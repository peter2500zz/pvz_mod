use mlua::prelude::*;

use crate::pvz::lawn_app::lawn_app::get_lawn_app;

#[derive(Debug)]
#[repr(C)]
/// 这是 `WidgetManager`
pub struct WidgetManager {
    _pad_0x0_0xE0: [u8; 0xE0 - 0x0],
    /// 0xE0 鼠标横坐标
    pub mouse_x: i32,
    /// 0xE4 鼠标纵坐标
    pub mouse_y: i32,
    _pad_0xE8_0x1FC: [u8; 0x1FC - 0xE8],
}
const _: () = assert!(size_of::<WidgetManager>() == 0x1FC);

pub fn get_widget_manager() -> Option<*mut WidgetManager> {
    unsafe {
        get_lawn_app().and_then(|lawn_app| {
            if ((*lawn_app).widget_manager as u32) == 0 {
                None
            } else {
                Some((*lawn_app).widget_manager)
            }
        })
    }
}

pub fn with_widget_manager<T>(f: impl FnOnce(&mut WidgetManager) -> T) -> LuaResult<T> {
    get_widget_manager()
        .map(|widget_manager| unsafe { f(&mut *widget_manager) })
        .ok_or_else(|| LuaError::MemoryError("WidgetManager 不可访问".to_string()))
}

impl LuaUserData for WidgetManager {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetMousePos", |_, _, ()| {
            with_widget_manager(|wm| {
                (wm.mouse_x, wm.mouse_y)
            })
        });
    }
}
