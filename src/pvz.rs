pub mod board;
pub mod coin;
pub mod debug;
pub mod graphics;
pub mod lawn_app;
pub mod widget_manager;
pub mod widget_container;
pub mod zombie;
pub mod resource_manager;

use tracing::{debug, info};
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

    debug!("启动参数: {}", if args.is_empty() { "无" } else { &args });
    info!("主程序启动");
    debug!("句柄 {:#x?}", hInstance.0);

    let result = ORIGINAL_WINMAIN.wait()(hInstance, hPrevInstance, lpCmdLine, nCmdShow);

    info!("主程序终止");
    debug!("退出代码 {:#x}", result);

    pause!("请按任意键继续. . .");

    result
}
