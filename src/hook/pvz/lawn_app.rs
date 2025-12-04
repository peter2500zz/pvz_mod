pub mod loading;

use std::{ffi::c_void, sync::OnceLock};

use super::{HookRegistration, hook};
use crate::pvz::lawn_app::{self, LawnApp};

/// `LawnApp` 构造函数的地址
const ADDR_LAWNAPP_CONSTRUCTOR: *mut c_void = 0x0044EAA0 as _;
/// `LawnApp` 构造函数的签名
type SignLawnAppConstructor = extern "stdcall" fn(
    uninit: *mut LawnApp
) -> *mut LawnApp;
/// `LawnApp` 构造函数的跳板
pub static ORIGINAL_LAWNAPP_CONSTRUCTOR: OnceLock<SignLawnAppConstructor> = OnceLock::new();

/// `LawnApp` 析构函数的地址
const ADDR_LAWNAPP_DESTRUCTOR: *mut c_void = 0x0044EDF0 as _;
/// `LawnApp` 析构函数的签名
type SignLawnAppDestructor = extern "thiscall" fn(
    this: *mut LawnApp
);
/// `LawnApp` 析构函数的跳板
pub static ORIGINAL_LAWNAPP_DESTRUCTOR: OnceLock<SignLawnAppDestructor> = OnceLock::new();

/// `LawnApp::Init` 的地址
const ADDR_LAWNAPP_INIT: *mut c_void = 0x00451880 as _;
/// `LawnApp::Init` 的签名
type SignLawnAppInit = extern "thiscall" fn(
    this: *mut LawnApp
);
/// `LawnApp::Init` 的跳板
pub static ORIGINAL_LAWNAPP_INIT: OnceLock<SignLawnAppInit> = OnceLock::new();

/// `LawnApp::LostFocus` 的地址
const ADDR_LAWNAPP_LOST_FOCUS: *mut c_void = 0x0044F460 as _;
/// `LawnApp::LostFocus` 的签名
type SignLawnAppLostFocus = extern "thiscall" fn(
    this: *mut LawnApp
);
/// `LawnApp::LostFocus` 的跳板
pub static ORIGINAL_LAWNAPP_LOST_FOCUS: OnceLock<SignLawnAppLostFocus> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_LAWNAPP_CONSTRUCTOR.set(
            hook(ADDR_LAWNAPP_CONSTRUCTOR, lawn_app::Constructor as _)?
        );

        let _ = ORIGINAL_LAWNAPP_DESTRUCTOR.set(
            hook(ADDR_LAWNAPP_DESTRUCTOR, lawn_app::Destructor as _)?
        );

        let _ = ORIGINAL_LAWNAPP_INIT.set(
            hook(ADDR_LAWNAPP_INIT, lawn_app::Init as _)?
        );

        let _ = ORIGINAL_LAWNAPP_LOST_FOCUS.set(
            hook(ADDR_LAWNAPP_LOST_FOCUS, lawn_app::LostFocus as _)?
        );

        Ok(())
    })
}
