use mlua::prelude::*;
use tracing::trace;

use crate::mods::with_lua;

use super::LuaRegistration;

pub const PRE: u32 = 0 << 31;
pub const POST: u32 = 1 << 31;

#[macro_export]
macro_rules! add_callback {
    ($name:literal, $addr:expr) => {
        inventory::submit! {
            $crate::mods::LuaRegistration(|lua| {
                let globals = lua.globals();

                globals.set("ModCallbacks", globals.get("ModCallbacks").unwrap_or(lua.create_table()?))?;
                let mod_callbacks: mlua::Table = globals.get("ModCallbacks")?;

                mod_callbacks.set(format!("PRE_{}", $name.to_uppercase()), PRE | $addr)?;
                mod_callbacks.set(format!("POST_{}", $name.to_uppercase()), POST | $addr)?;

                Ok(())
            })
        }
    };
}

#[macro_export]
macro_rules! callback {
    ($addr:expr, ($($item:expr),*)) => {
        
        ($($item,)*)
    };
}

inventory::submit! {
    LuaRegistration(|lua| {
        // 取得全局变量表
        let globals = lua.globals();

        let register_mod = lua.create_function(|lua, name: String| {
            let the_mod = lua.create_table()?;

            the_mod.set("name", name)?;

            let add_callback_func = lua.create_function(|lua, (this, callback, function): (LuaTable, u32, LuaFunction)| {
                // 获取 mod 信息
                let name: String = this.get("name")?;

                // 取得全局变量表
                let globals = lua.globals();

                // 获取回调函数表
                globals.set("Callbacks", globals.get("Callbacks").unwrap_or(lua.create_table()?))?;
                let callbacks: LuaTable = globals.get("Callbacks")?;

                // 获取回调点表
                callbacks.set(callback, callbacks.get(callback).unwrap_or(lua.create_table()?))?;
                let callback_point: LuaTable = callbacks.get(callback)?;

                trace!("Mod({}) 添加了回调函数，位置 {}", &name, format!("{} 0x{:08x}", if (callback >> 31) == 0 { "Pre" } else { "Post" }, (callback << 1 >> 1)));

                callback_point.raw_push(function)?;

                Ok(())
            })?;

            the_mod.set("AddCallback", add_callback_func)?;

            Ok(the_mod)
        })?;

        globals.set("RegisterMod", register_mod)?;

        Ok(())
    })
}
