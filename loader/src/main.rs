#![windows_subsystem = "windows"]

mod detours;

use std::{ffi::c_void, path::{Path, PathBuf}, ptr::null_mut};
use anyhow::{Result, anyhow};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use windows::{
    Win32::{
        Storage::FileSystem::{
            GetFileVersionInfoSizeW, GetFileVersionInfoW, VS_FIXEDFILEINFO, VerQueryValueW,
        },
        UI::WindowsAndMessaging::{
            IDCANCEL, IDNO, IDYES, MB_DEFBUTTON1, MB_ICONWARNING, MB_YESNOCANCEL,
            MESSAGEBOX_RESULT, MessageBoxW,
        },
    },
    core::{PCWSTR, w},
};
use windows_wrapper::{formatw, mb};
use config::{load_config, save_config};

use crate::detours::launch_pvz;

const CONFIG: &str = "conf.yml";
const TARGET_VERSION: &str = "1.0.0.1051";
const DEFAULT_GAME_BIN_NAME: &str = "PlantsVsZombies.exe";
const TARGET_PRODUCT_NAME: &str = "Plants vs. Zombies";

#[derive(Serialize, Deserialize, Debug)]
struct LoaderConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_launch: Option<bool>,
}

fn main() {
    let mut config: LoaderConfig = load_config(CONFIG);

    let mut path = if let Some(path) = config.path.clone() && Path::new(&path).exists() {
        path
    } else {
        let path = get_game_exe().to_string_lossy().to_string();

        config.path = Some(path.clone());
        save_config(CONFIG, &config);

        path
    };

    loop {
        if config.force_launch.unwrap_or(false) {
            break;
        }

        match check_version(&path) {
            Ok(_) => break,
            Err(e) => match continue_or_not(&e.to_string()) {
                IDYES => {
                    config.force_launch = Some(true);
                    save_config(CONFIG, &config);

                    break
                },
                IDNO => {
                    path = get_game_exe().to_string_lossy().to_string();

                    config.path = Some(path.clone());
                    save_config(CONFIG, &config);
                }
                IDCANCEL | _ => return,
            },
        }
    }

    let result = launch_pvz(path);

    if result.is_none() {
        config.force_launch = Some(false);
        save_config(CONFIG, &config);

        mb!("启动失败，已禁用强制启动。你可以在下次启动时选择其他可执行文件。");
    }
}

fn continue_or_not(msg: &str) -> MESSAGEBOX_RESULT {
    unsafe {
        let msgu16 = formatw!(
            "Rumia 只能运行在《植物大战僵尸》v{} 版，但{}。\n你想要强行启动吗？点击“否”以选择另一个游戏文件。",
            TARGET_VERSION,
            msg
        );

        MessageBoxW(
            None,
            PCWSTR(msgu16.as_ptr() as _),
            w!("《植物大战僵尸》游戏文件无效！"),
            MB_YESNOCANCEL | MB_ICONWARNING | MB_DEFBUTTON1,
        )
    }
}

fn get_game_exe() -> PathBuf {
    let default_path = PathBuf::from(DEFAULT_GAME_BIN_NAME);
    if default_path.exists() {
        return default_path;
    }

    FileDialog::new()
        .add_filter("可执行文件", &["exe"])
        .add_filter("所有文件", &["*"])
        .set_directory(".")
        .pick_file()
        .expect("未选择文件")
}

fn check_version(path: &str) -> Result<String> {
    unsafe {
        // 得绑定一下变量不然就吊死了
        let pathu16 = formatw!("{}", path);
        let lp_pathu16 = PCWSTR(pathu16.as_ptr());

        let mut handle = 0;

        let size = GetFileVersionInfoSizeW(lp_pathu16, Some(&mut handle));
        if size == 0 {
            return Err(anyhow!("无法获取 {} 的版本信息", path));
        }

        let mut buffer: Vec<u8> = vec![0u8; size as usize];

        GetFileVersionInfoW(
            lp_pathu16,
            Some(handle),
            size,
            buffer.as_mut_ptr() as *mut c_void,
        )?;

        // 检查产品名称
        let mut ptr = null_mut();
        let mut len: u32 = 0;

        // 查询产品名称 (需要先查询语言代码页)
        let success = VerQueryValueW(
            buffer.as_ptr() as *const c_void,
            w!("\\VarFileInfo\\Translation"),
            &mut ptr as *mut *mut c_void,
            &mut len,
        );

        if success.as_bool() && len >= 4 {
            let translation = ptr as *const u16;
            let lang = *translation;
            let codepage = *translation.offset(1);

            // 构造查询字符串,格式: \StringFileInfo\<lang><codepage>\ProductName
            let query = formatw!(
                "\\StringFileInfo\\{:04x}{:04x}\\ProductName",
                lang,
                codepage
            );

            let mut product_ptr = null_mut();
            let mut product_len: u32 = 0;

            let success = VerQueryValueW(
                buffer.as_ptr() as *const c_void,
                PCWSTR(query.as_ptr()),
                &mut product_ptr as *mut *mut c_void,
                &mut product_len,
            );

            if success.as_bool() && product_len > 0 {
                let product_name_ptr = product_ptr as *const u16;
                let product_name = String::from_utf16_lossy(std::slice::from_raw_parts(
                    product_name_ptr,
                    (product_len - 1) as usize,
                ));

                if product_name != TARGET_PRODUCT_NAME {
                    return Err(anyhow!("这个程序的看起来是 {}", product_name));
                }
            }
        }

        // 检查版本号
        ptr = null_mut();
        len = 0;

        let success = VerQueryValueW(
            buffer.as_ptr() as *const c_void,
            w!("\\"),
            &mut ptr as *mut *mut c_void,
            &mut len,
        );

        if !success.as_bool() {
            return Err(anyhow!("无法解析 {} 的版本值", path));
        }

        let info = &*(ptr as *const VS_FIXEDFILEINFO);

        let major = (info.dwFileVersionMS >> 16) & 0xFFFF;
        let minor = info.dwFileVersionMS & 0xFFFF;
        let build = (info.dwFileVersionLS >> 16) & 0xFFFF;
        let revision = info.dwFileVersionLS & 0xFFFF;

        let version_str = format!("{}.{}.{}.{}", major, minor, build, revision);

        if version_str != TARGET_VERSION {
            return Err(anyhow!("你选择的游戏版本为 v{}", version_str));
        }

        Ok(version_str)
    }
}
