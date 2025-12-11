pub mod widget_manager;

use tracing::trace;

use crate::{
    add_callback,
    hook::pvz::widget_manager::{
        ADDR_KEY_DOWN, ADDR_KEY_UP, ADDR_POST_DRAW_SCREEN, KeyDownWrapper, KeyUpWrapper,
        ORIGINAL_CONSTRUCTOR, ORIGINAL_DESTRUCTOR,
    },
    mods::callback::{POST, PRE, callback},
    pvz::{
        graphics::graphics::{Graphics, GraphicsHandle},
        lawn_app::lawn_app::LawnApp,
        widget_manager::widget_manager::WidgetManager,
    },
};

/// 这是 `WidgetManager` 的构造函数
pub extern "stdcall" fn Constructor(
    uninit: *mut WidgetManager,
    theApp: *mut LawnApp,
) -> *mut WidgetManager {
    trace!("构造 WidgetManager");

    let this = ORIGINAL_CONSTRUCTOR.wait()(uninit, theApp);

    trace!("地址 {:#x?}", this);

    this
}

/// 这是 `WidgetManager` 的析构函数
pub extern "thiscall" fn Destructor(this: *mut WidgetManager) {
    trace!("析构 WidgetManager");

    ORIGINAL_DESTRUCTOR.wait()(this);
}

pub extern "stdcall" fn KeyDown(this: *mut WidgetManager, key: i32) {
    // trace!("按下键码 {:#x}", key);
    if !callback(PRE | ADDR_KEY_DOWN, key) {
        KeyDownWrapper(this, key);
    }
}
add_callback!("AT_GAME_KEY_DOWN", PRE | ADDR_KEY_DOWN);

pub extern "stdcall" fn KeyUp(this: *mut WidgetManager, key: i32) {
    // trace!("松开键码 {:#x}", key);
    if !callback(PRE | ADDR_KEY_UP, key) {
        KeyUpWrapper(this, key);
    }
}
add_callback!("AT_GAME_KEY_UP", PRE | ADDR_KEY_UP);

pub extern "thiscall" fn PostDrawScreen(g: *mut Graphics) {
    // 不要使用 ptr::read(g)！
    // 直接创建一个 Handle，传递指针
    // GraphicsHandle 实现了 Copy/Clone，可以直接传给 callback
    callback(POST | ADDR_POST_DRAW_SCREEN, GraphicsHandle(g));
}
add_callback!("AT_GAME_DRAW", POST | ADDR_POST_DRAW_SCREEN);
