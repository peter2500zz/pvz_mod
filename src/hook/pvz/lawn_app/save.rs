use std::{
    arch::{asm, naked_asm},
    sync::atomic::{AtomicUsize, Ordering},
};

use super::{HookRegistration, hook};
use crate::{pvz::lawn_app::save::GetAppDataFolder, utils::msvc_string::MsvcString};

/// `Sexy::GetAppDataFolder` 的地址
const ADDR_GET_APP_DATA_FOLDER: u32 = 0x005AF590 as _;
/// `Sexy::GetAppDataFolder` 的跳板
pub static ORIGINAL_GET_APP_DATA_FOLDER: AtomicUsize = AtomicUsize::new(0);

#[unsafe(naked)]
extern "stdcall" fn GetAppDataFolderHelper() {
    naked_asm!(
        "push esi",
        "call {func}",
        "ret",
        func = sym GetAppDataFolder
    )
}

pub extern "stdcall" fn GetAppDataFolderWrapper(string: *mut MsvcString) -> *mut MsvcString {
    unsafe {
        let result: *mut MsvcString;
        asm!(
            "push esi",
            "mov esi, {string}",

            "call [{func}]",
            "pop esi",

            string = in(reg) string,

            func = sym ORIGINAL_GET_APP_DATA_FOLDER,

            lateout("eax") result,

            clobber_abi("C")
        );
        result
    }
}

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_GET_APP_DATA_FOLDER.store(
            hook(ADDR_GET_APP_DATA_FOLDER as _, GetAppDataFolderHelper as _)?, Ordering::SeqCst
        );

        Ok(())
    })
}
