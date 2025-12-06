mod register;

use anyhow::Result;
use std::{fs::{self, DirEntry}, sync::{Arc, Mutex, LazyLock}};
use tracing::{info, error};
use mlua::{Lua, Result as LuaResult};
use regex::Regex;

const MOD_DIR: &str = "mods";
const MAIN_FILE: &str = "main.lua";

type LuaInitFn = fn(&mut Lua) -> LuaResult<()>;
struct LuaRegistration(LuaInitFn);

inventory::collect!(LuaRegistration);

static LUA: LazyLock<Arc<Mutex<Lua>>> = LazyLock::new(|| {
    info!("初始化 Lua 状态机");

    let mut lua = Lua::new();

    for LuaRegistration(lua_init) in inventory::iter::<LuaRegistration> {
        if let Err(e) = lua_init(&mut lua) {
            error!("Lua 初始化时出现错误");
            panic!("Lua 初始化时出现错误: {}", e);
        }
    }

    Arc::new(Mutex::new(lua))
});

static EXTRACT: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(r":\s(.*?):\s").ok()
});

pub fn with_lua<F>(exec: F) -> LuaResult<()>
where
    F: FnOnce(&mut Lua) -> LuaResult<()>
{
    match LUA.lock() {
        Ok(mut lua) => {
            let result = exec(&mut lua);

            if let Err(e) = &result {
                let error = if let Some(extract) = EXTRACT.as_ref() {
                    extract.replace(&e.to_string(), ": ").to_string()
                } else {
                    e.to_string()
                };

                error!("{}", error);
            }

            result
        },
        Err(e) => {
            error!("Lua 状态机错误: {}", e);
            panic!("Lua 状态机错误: {}", e)
        }
    }
}

pub fn load_mods() -> Result<u32> {
    let mut success = 0;

    for entry in fs::read_dir(MOD_DIR)? {
        if let Ok(_) = load_mod(entry) {
            success += 1;
        }
    }

    Ok(success)
}

fn load_mod(entry: Result<DirEntry, std::io::Error>) -> Result<()> {
    let path = entry?.path();

    if !path.is_dir() { return Ok(()) };

    let main_file = path.join(MAIN_FILE);

    let script = fs::read_to_string(&main_file)?;

    let result = with_lua(|lua| {
        lua.load(script).exec()?;

        Ok(())
    });

    if let Err(e) = result {
        error!("加载 Mod({}) 时出现问题", &path.to_string_lossy());
        panic!("加载 Mod({}) 时出现问题: {}", &path.to_string_lossy(), e);
    }

    Ok(())
}
