pub mod lawn_app;
pub mod debug;
pub mod board;
pub mod widget_manager;
pub mod widget_container;
pub mod zombie;
pub mod coin;
pub mod graphics;

use inventory;
use std::sync::OnceLock;
use windows::{
    Win32::{Foundation::HINSTANCE, UI::WindowsAndMessaging::SHOW_WINDOW_CMD},
    core::PSTR,
};

use super::{HookRegistration, hook};
use crate::pvz;

/// 主函数的地址
const ADDR_WINMAIN: u32 = 0x0044E8F0 as _;
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
        let _ = ORIGINAL_WINMAIN.set(hook(ADDR_WINMAIN as _, pvz::WinMain as _)?);

        Ok(())
    })
}
