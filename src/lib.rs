use std::ffi::c_void;
use windows::{
    Win32::{
        Foundation::HINSTANCE, 
        System::SystemServices::{
            DLL_PROCESS_ATTACH, 
            DLL_PROCESS_DETACH, 
            DLL_THREAD_ATTACH, 
            DLL_THREAD_DETACH
        }
    }, 
    core::BOOL
};


#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(hinstDLL: HINSTANCE, fdwReason: u32, lpReserved: *mut c_void) -> BOOL {
    // just satisfy clippy
    let _ = hinstDLL;
    let _ = lpReserved;

    match fdwReason {
        DLL_PROCESS_ATTACH => {
            
        },
        DLL_PROCESS_DETACH => {

        },
        DLL_THREAD_ATTACH => {

        },
        DLL_THREAD_DETACH => {

        },

        _ => unreachable!()
    }

    BOOL::from(true)
}