pub mod widget_manager;

use tracing::trace;

use crate::{
    hook::pvz::widget_manager::{
        KeyDownWrapper, ORIGINAL_CONSTRUCTOR, ORIGINAL_DESTRUCTOR
    }, 
    pvz::{lawn_app::lawn_app::LawnApp, widget_manager::widget_manager::WidgetManager}
};


/// 这是 `WidgetManager` 的构造函数
pub extern "stdcall" fn Constructor(
    uninit: *mut WidgetManager, 
    theApp: *mut LawnApp
) -> *mut WidgetManager {
    trace!("构造 WidgetManager");

    let this = ORIGINAL_CONSTRUCTOR.wait()(
        uninit,
        theApp
    );

    trace!("地址 {:#x?}", this);

    this
}

/// 这是 `WidgetManager` 的析构函数
pub extern "thiscall" fn Destructor(
    this: *mut WidgetManager
) {
    trace!("析构 WidgetManager");

    ORIGINAL_DESTRUCTOR.wait()(this);
}

pub extern "stdcall" fn KeyDown(
    this: *mut WidgetManager,
    key: i32,
) -> u8 {
    trace!("按下键码 {:#x}", key);

    KeyDownWrapper(this, key)
}
