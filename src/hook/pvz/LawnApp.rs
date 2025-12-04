use std::{ffi::c_void, sync::OnceLock};

use super::{HookRegistration, hook};
use crate::pvz::LawnApp;

const ADDR_LAWNAPP_CONSTRUCTOR: *mut c_void = 0x0044EAA0 as _;
type SignLawnAppConstructor =
    extern "stdcall" fn(uninit: *mut LawnApp::LawnApp) -> *mut LawnApp::LawnApp;
pub static ORIGINAL_LAWNAPP_CONSTRUCTOR: OnceLock<SignLawnAppConstructor> = OnceLock::new();

const ADDR_LAWNAPP_DESTRUCTOR: *mut c_void = 0x0044EDF0 as _;
type SignLawnAppDestructor = extern "thiscall" fn(this: *mut LawnApp::LawnApp);
pub static ORIGINAL_LAWNAPP_DESTRUCTOR: OnceLock<SignLawnAppDestructor> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_LAWNAPP_CONSTRUCTOR.set(
            hook(ADDR_LAWNAPP_CONSTRUCTOR, LawnApp::Constructor as _)?
        );

        let _ = ORIGINAL_LAWNAPP_DESTRUCTOR.set(
            hook(ADDR_LAWNAPP_DESTRUCTOR, LawnApp::Destructor as _)?
        );

        Ok(())
    })
}
