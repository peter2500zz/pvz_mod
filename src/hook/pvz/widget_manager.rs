
use std::{ffi::c_void, sync::OnceLock};

use crate::pvz::{lawn_app::LawnApp, widget_manager::{self, WidgetManager}};
use super::{HookRegistration, hook};


/// `WidgetManager` 构造函数的地址
const ADDR_WIDGET_MANAGER_CONSTRUCTOR: *mut c_void = 0x005384A0 as _;
/// `WidgetManager` 构造函数的签名
type SignWidgetManagerConstructor = extern "stdcall" fn(
    uninit: *mut WidgetManager,
    theApp: *mut LawnApp,
) -> *mut WidgetManager;
/// `WidgetManager` 构造函数的跳板
pub static ORIGINAL_WIDGET_MANAGER_CONSTRUCTOR: OnceLock<SignWidgetManagerConstructor> = OnceLock::new();

/// `WidgetManager` 析构函数的地址
const ADDR_WIDGET_MANAGER_DESTRUCTOR: *mut c_void = 0x00538610 as _;
/// `WidgetManager` 析构函数的签名
type SignWidgetManagerDestructor = extern "thiscall" fn(
    this: *mut WidgetManager
);
/// `WidgetManager` 析构函数的跳板
pub static ORIGINAL_WIDGET_MANAGER_DESTRUCTOR: OnceLock<SignWidgetManagerDestructor> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_WIDGET_MANAGER_CONSTRUCTOR.set(
            hook(ADDR_WIDGET_MANAGER_CONSTRUCTOR, widget_manager::Constructor as _)?
        );

        let _ = ORIGINAL_WIDGET_MANAGER_DESTRUCTOR.set(
            hook(ADDR_WIDGET_MANAGER_DESTRUCTOR, widget_manager::Destructor as _)?
        );

        Ok(())
    })
}
