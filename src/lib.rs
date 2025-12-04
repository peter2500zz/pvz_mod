mod debug;
mod hook;
mod logger;
#[allow(non_snake_case)]
mod pvz;
mod utils;

use anyhow::Result;
use std::ffi::c_void;
use tracing::error;
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
        DLL_PROCESS_ATTACH => on_pocess_attach(hinstDLL).is_ok(),
        DLL_PROCESS_DETACH => true,
        DLL_THREAD_ATTACH => true,
        DLL_THREAD_DETACH => true,

        _ => unreachable!(),
    };

    BOOL::from(result)
}

fn on_pocess_attach(handle: HINSTANCE) -> Result<()> {
    mb!("attached successfully\nwith handle: {:#x?}", &handle.0);

    alloc_console()?;
    setup_logger()?;

    if let Err(e) = init_hook() {
        error!("error when init hook: {}", e);
    }

    Ok(())
}
