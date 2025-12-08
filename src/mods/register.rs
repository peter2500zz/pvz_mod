use mlua::prelude::*;
use tracing::trace;

use crate::mods::with_lua;

use super::LuaRegistration;


#[macro_export]
macro_rules! add_callback {
    ($name:literal, $addr:expr) => {
        inventory::submit! {
            $crate::mods::LuaRegistration(|lua| {
                let globals = lua.globals();

                globals.set("ModCallbacks", globals.get("ModCallbacks").unwrap_or(lua.create_table()?))?;
                let mod_callbacks: mlua::Table = globals.get("ModCallbacks")?;

                mod_callbacks.set(format!("{}", $name.to_uppercase()), $addr)?;

                Ok(())
            })
        }
    };
}

// pub fn callback<T>(at: u32, args: T) -> T
// where 
//     T: IntoLuaMulti + FromLuaMulti + Clone,
// {
//     let result_callback_funcs = with_lua(|lua| {
//         // 取得全局变量表
//         let globals = lua.globals();

//         // 获取回调函数表
//         globals.set("Callbacks", globals.get("Callbacks").unwrap_or(lua.create_table()?))?;
//         let callbacks: LuaTable = globals.get("Callbacks")?;

//         // 获取回调点表
//         callbacks.set(at, callbacks.get(at).unwrap_or(lua.create_table()?))?;
//         let callback_point: LuaTable = callbacks.get(at)?;

//         let pairs = callback_point.pairs::<i32, LuaFunction>();

//         let mut funcs = Vec::new();

//         for pair in pairs {
//             let (_, func) = pair?;

//             funcs.push(func);
//         }

//         Ok(funcs)
//     });
    
//     match result_callback_funcs {
//         Ok(callback_funcs) => {
//             let mut args = args;
//             for callback_func in callback_funcs {
//                 let args_clone = args.clone();
//                 if let Ok(result) = callback_func.call(args_clone) {
//                     args = result;
//                 }
//             }
//             args
//         },
//         Err(_) => args
//     }
// }

// 修改点 1: 这里的 args 类型变为 &mut T，且不再有返回值
// 约束 T 必须实现 UserData (因为我们要把它暴露给 Lua)
pub fn callback<T>(at: u32, args: &mut T)
where 
    T: LuaUserData + 'static,
{
    // 假设这是你获取全局 Lua 实例的方式
    let _ = with_lua(|lua| {
        let globals = lua.globals();

        // --- 步骤 1: 获取回调函数列表 (这部分逻辑基本不变) ---
        // 查找 Callbacks 表
        let callbacks: LuaTable = match globals.get("Callbacks") {
            Ok(t) => t,
            Err(_) => {
                let t = lua.create_table()?;
                globals.set("Callbacks", &t)?;
                t
            }
        };

        // 查找对应的 Hook 点 (at)
        let callback_point: LuaTable = match callbacks.get(at) {
            Ok(t) => t,
            Err(_) => {
                let t = lua.create_table()?;
                callbacks.set(at, &t)?;
                t
            }
        };

        // 收集所有回调函数
        let pairs = callback_point.pairs::<i32, LuaFunction>();
        let mut funcs = Vec::new();
        for pair in pairs {
            if let Ok((_, func)) = pair {
                funcs.push(func);
            }
        }

        // --- 步骤 2: 核心修改 - 使用 Scope 传递引用 ---
        // 使用 scope 是为了绕过 Rust 的生命周期检查，安全地将 &mut T 借给 Lua
        lua.scope(|scope| {
            // 关键：创建一个指向 args 的可变 UserData 引用
            // 这不会发生 Clone，它只是包装了指针
            let userdata_ref = scope.create_userdata_ref_mut(args)?;

            for func in funcs {
                // 调用 Lua 函数
                // 我们传入 userdata_ref (它是一个 Handle，clone 它很廉价，不会复制底层数据)
                // 这里的范型 <_, ()> 表示参数是 UserData，返回值为空(我们不需要 Lua 返回任何东西)
                if let Err(e) = func.call::<()>(userdata_ref.clone()) {
                    eprintln!("Lua callback execution failed: {}", e);
                    // 即使某个回调报错，是否继续执行下一个？这里选择了继续，你可以加 break
                }
            }
            Ok(())
        })
    });
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

                trace!("Mod({}) 添加了回调函数，位置 0x{:08x}", &name, callback);

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
