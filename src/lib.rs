mod debug;
mod hook;
mod logger;
mod mods;
#[allow(non_snake_case)]
mod pvz;
mod utils;
mod config;
mod save;

use anyhow::Result;
use std::ffi::c_void;
use tracing::{debug, info};
use windows::{
    Win32::{
        Foundation::HINSTANCE,
        System::SystemServices::{
            DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
        },
    },
    core::BOOL,
};
use windows_wrapper::mb;

use crate::{debug::alloc_console, hook::init_hook, logger::setup_logger};

const CONFIG: &str = "conf.yml";

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(
    hinstDLL: HINSTANCE,
    fdwReason: u32,
    lpReserved: *mut c_void,
) -> BOOL {
    // just satisfy clippy
    let _ = lpReserved;

    let result = match fdwReason {
        DLL_PROCESS_ATTACH => match on_pocess_attach(hinstDLL) {
            Ok(_) => {
                info!("初始化成功");

                true
            }
            Err(e) => {
                mb!("初始化时遇到问题\n{}", e);

                false
            }
        },
        DLL_PROCESS_DETACH => true,
        DLL_THREAD_ATTACH => true,
        DLL_THREAD_DETACH => true,

        _ => unreachable!(),
    };

    BOOL::from(result)
}

fn on_pocess_attach(handle: HINSTANCE) -> Result<()> {
    alloc_console()?;
    setup_logger()?;

    info!("DLL 注入成功");
    debug!("句柄 {:#x?}", handle.0);

    init_hook()?;

    info!("Hook 成功");

    Ok(())
}
