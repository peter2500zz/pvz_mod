pub mod LawnApp;

use tracing::info;
use windows::{
    Win32::{Foundation::HINSTANCE, UI::WindowsAndMessaging::SHOW_WINDOW_CMD},
    core::PSTR,
};

use crate::{hook::pvz::ORIGINAL_WINMAIN, pause};

pub extern "stdcall" fn WinMain(
    hInstance: HINSTANCE,
    hPrevInstance: HINSTANCE,
    lpCmdLine: PSTR,
    nCmdShow: SHOW_WINDOW_CMD,
) -> i32 {
    let args = unsafe { lpCmdLine.to_string().unwrap_or_default() };

    info!("启动参数: {}", args);

    let result = ORIGINAL_WINMAIN.wait()(hInstance, hPrevInstance, lpCmdLine, nCmdShow);

    info!("终止代码: {:#x}", result);

    pause!("请按任意键继续. . .");

    result
}
