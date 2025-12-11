use mlua::prelude::*;

use super::log::*;
use crate::mods::with_lua;

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

                mod_callbacks.set(format!("{}", $name.to_uppercase()), $addr)?;

                Ok(())
            })
        }
    };
}

/// 通用回调函数 - 支持传递任意实现 IntoLuaMulti 的参数
/// 
/// 如果返回了 `true` 将不会执行原函数
pub fn callback<A>(at: u32, args: A) -> bool
where 
    A: IntoLuaMulti + Clone,
{
    let result = with_lua(|lua| {
        let globals = lua.globals();

        // 获取或创建 Callbacks 表
        let callbacks: LuaTable = match globals.get("Callbacks") {
            Ok(t) => t,
            Err(_) => {
                let t = lua.create_table()?;
                globals.set("Callbacks", &t)?;
                t
            }
        };

        // 获取或创建对应 Hook 点的回调列表
        let callback_point: LuaTable = match callbacks.get(at) {
            Ok(t) => t,
            Err(_) => {
                let t = lua.create_table()?;
                callbacks.set(at, &t)?;
                t
            }
        };

        // 收集所有回调函数
        let mut funcs = Vec::new();
        for pair in callback_point.pairs::<i32, LuaFunction>() {
            if let Ok((_, func)) = pair {
                funcs.push(func);
            }
        }

        // 执行所有回调
        for func in funcs {
            // 每次调用都 clone args (因为 IntoLuaMulti 会消耗参数)
            match func.call::<bool>(args.clone()) {
                Ok(result) => if result { return Ok(result); },
                Err(e) => error!("Lua callback execution failed: {}", e),
            }
        }

        Ok(false)
    });
    if let Ok(result) = result {
        result
    } else {
        false
    }
}

// 修改点 1: 这里的 args 类型变为 &mut T，且不再有返回值
// 约束 T 必须实现 UserData (因为我们要把它暴露给 Lua)
pub fn callback_data<T>(at: u32, args: &mut T)
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
                    error!("Lua callback execution failed: {}", e);
                    // 即使某个回调报错，是否继续执行下一个？这里选择了继续，你可以加 break
                }
            }
            Ok(())
        })
    });
}