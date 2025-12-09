use std::ptr;

use mlua::prelude::*;

use crate::{mods::LuaRegistration, pvz::board::board::Board};

const ADDR_LAWN_APP_BASE: u32 = 0x006A9EC0;

pub fn get_lawn_app() -> Option<*mut LawnApp> {
    unsafe {
        if (*(ADDR_LAWN_APP_BASE as *const u32)) == 0 {
            None
        } else {
            Some(*(ADDR_LAWN_APP_BASE as *const *mut LawnApp))
        }
    }
}

pub fn with_lawn_app<T>(f: impl FnOnce(&mut LawnApp) -> T) -> LuaResult<T> {
    get_lawn_app()
        .map(|lawn_app| unsafe { f(&mut *lawn_app) })
        .ok_or_else(|| LuaError::MemoryError("LawnApp 不可访问".to_string()))
}

inventory::submit! {
    LuaRegistration(|lua| {
        let globals = lua.globals();

        let lua_get_lawn_app = lua.create_function(move |lua, ()| {
            if let Some(p_lawn_app) = get_lawn_app() {
                unsafe {
                    // 强制读取里面的东西
                    let lawn_app = lua.create_userdata(ptr::read(p_lawn_app))?;

                    Ok(mlua::Value::UserData(lawn_app))
                }
            } else {
                Ok(mlua::Value::Nil)
            }
        })?;

        globals.set("LawnApp", lua_get_lawn_app)?;

        Ok(())
    })
}

#[derive(Debug)]
#[repr(C)]
/// 这是 `LawnApp`
/// 
/// 手动管理生命周期并不好玩，孩子们
pub struct LawnApp {
    _pad_0x0_0xC0: [u8; 0xC0 - 0x0],
    /// 0xC0 窗口宽
    pub window_width: u32,
    /// 0xC4 窗口高
    pub window_height: u32,
    _pad_0xC8_0x768: [u8; 0x768 - 0xC8],
    /// 0x768 游戏关卡
    pub board: *mut Board,
    _pad_0x76C_0x8C8: [u8; 0x8C8 - 0x76C],
}
const _: () = assert!(size_of::<LawnApp>() == 0x8C8);

impl LuaUserData for LawnApp {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // 获取窗口尺寸
        methods.add_method("windowSize", |_, _, ()| {
            with_lawn_app(|lawn_app| {
                (lawn_app.window_width, lawn_app.window_height)
            })
        });
        
        // 获取关卡类
        methods.add_method("board", |lua, this, ()| {
            if this.board as u32 == 0 {
                Ok(LuaNil)
            } else {
                unsafe {
                    let lawn_app = lua.create_userdata(ptr::read(this.board))?;

                    Ok(mlua::Value::UserData(lawn_app))
                }
            }
        });
    }
}
