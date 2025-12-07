
use std::{ffi::c_char, sync::OnceLock};

use super::{HookRegistration, hook};
use crate::pvz::lawn_app::{LawnApp, loading::{LoadGroup, LoadingThreadProc}};

/// `LawnApp::LoadingThreadProc` 的地址
const ADDR_LOADING_THREAD_PROC: u32 = 0x004528E0 as _;
/// `LawnApp::LoadingThreadProc` 的签名
type SignLoadingThreadProc =
    extern "thiscall" fn(this: *mut LawnApp);
/// `LawnApp::LoadingThreadProc` 的跳板
pub static ORIGINAL_LOADING_THREAD_PROC: OnceLock<SignLoadingThreadProc> = OnceLock::new();

/// `LawnApp::LoadingThreadProc` 的地址
const ADDR_LOAD_GROUP: u32 = 0x00452740 as _;
/// `LawnApp::LoadingThreadProc` 加载资源线程的签名
type SignLoadGroup = extern "thiscall" fn(
    this: *mut LawnApp, 
    theGroupName: *const c_char,
    theGroupAveMsToLoad: i32,
);
/// `LawnApp::LoadingThreadProc` 的跳板
pub static ORIGINAL_LOAD_GROUP: OnceLock<SignLoadGroup> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_LOADING_THREAD_PROC.set(
            hook(ADDR_LOADING_THREAD_PROC as _, LoadingThreadProc as _)?
        );

        let _ = ORIGINAL_LOAD_GROUP.set(
            hook(ADDR_LOAD_GROUP as _, LoadGroup as _)?
        );

        Ok(())
    })
}
