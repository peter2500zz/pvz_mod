// use tracing::trace;

use crate::{
    hook::pvz::lawn_app::save::GetAppDataFolderWrapper,
    utils::{get_exe_dir, msvc_string::MsvcString},
};

pub extern "stdcall" fn GetAppDataFolder(string: *mut MsvcString) -> *mut MsvcString {
    // 调用原始函数的 Wrapper
    let result = GetAppDataFolderWrapper(string);

    unsafe {
        if (*string).to_string() != "" {
            (*string).assign_all(
                &(get_exe_dir()
                    .join("saves")
                    .join("./")
                    .to_string_lossy()
                    .to_string())
                .into(),
            );
        }
        // trace!("获取游戏存档路径: {:?}", (*result).to_string());
    }

    result
}
