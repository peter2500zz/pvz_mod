pub mod loading;

use std::sync::OnceLock;

use super::{HookRegistration, hook};
use crate::pvz::lawn_app::{self, LawnApp};

/// `LawnApp` 构造函数的地址
const ADDR_CONSTRUCTOR: u32 = 0x0044EAA0 as _;
/// `LawnApp` 构造函数的签名
type SignConstructor = extern "stdcall" fn(
    uninit: *mut LawnApp
) -> *mut LawnApp;
/// `LawnApp` 构造函数的跳板
pub static ORIGINAL_CONSTRUCTOR: OnceLock<SignConstructor> = OnceLock::new();

/// `LawnApp` 析构函数的地址
const ADDR_DESTRUCTOR: u32 = 0x0044EDF0 as _;
/// `LawnApp` 析构函数的签名
type SignDestructor = extern "thiscall" fn(
    this: *mut LawnApp
);
/// `LawnApp` 析构函数的跳板
pub static ORIGINAL_DESTRUCTOR: OnceLock<SignDestructor> = OnceLock::new();

/// `LawnApp::Init` 的地址
const ADDR_INIT: u32 = 0x00451880 as _;
/// `LawnApp::Init` 的签名
type SignInit = extern "thiscall" fn(
    this: *mut LawnApp
);
/// `LawnApp::Init` 的跳板
pub static ORIGINAL_INIT: OnceLock<SignInit> = OnceLock::new();

/// `LawnApp::LostFocus` 的地址
const ADDR_LOST_FOCUS: u32 = 0x0044F460 as _;
/// `LawnApp::LostFocus` 的签名
type SignLostFocus = extern "thiscall" fn(
    this: *mut LawnApp
);
/// `LawnApp::LostFocus` 的跳板
pub static ORIGINAL_LOST_FOCUS: OnceLock<SignLostFocus> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_CONSTRUCTOR.set(
            hook(ADDR_CONSTRUCTOR as _, lawn_app::Constructor as _)?
        );

        let _ = ORIGINAL_DESTRUCTOR.set(
            hook(ADDR_DESTRUCTOR as _, lawn_app::Destructor as _)?
        );

        let _ = ORIGINAL_INIT.set(
            hook(ADDR_INIT as _, lawn_app::Init as _)?
        );

        let _ = ORIGINAL_LOST_FOCUS.set(
            hook(ADDR_LOST_FOCUS as _, lawn_app::LostFocus as _)?
        );

        Ok(())
    })
}
