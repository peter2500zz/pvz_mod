pub mod lawn_app;
pub mod debug;
pub mod board;
pub mod widget_manager;
pub mod zombie;

use inventory;
use std::{ffi::c_void, sync::OnceLock};
use windows::{
    Win32::{Foundation::HINSTANCE, UI::WindowsAndMessaging::SHOW_WINDOW_CMD},
    core::PSTR,
};

use super::{HookRegistration, hook};
use crate::pvz;

/// 主函数的地址
const ADDR_WINMAIN: *mut c_void = 0x0044E8F0 as _;
/// 主函数的签名
type SignWinMain = extern "stdcall" fn(
    hInstance: HINSTANCE,
    hPrevInstance: HINSTANCE,
    lpCmdLine: PSTR,
    nCmdShow: SHOW_WINDOW_CMD,
) -> i32;
/// 主函数的跳板
pub static ORIGINAL_WINMAIN: OnceLock<SignWinMain> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_WINMAIN.set(hook(ADDR_WINMAIN, pvz::WinMain as _)?);

        Ok(())
    })
}
