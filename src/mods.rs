pub mod register;
mod log;

use anyhow::{Context, Result}; // 建议引入 Context 方便报错
use std::{
    fs::{self, DirEntry},
    path::Path,
    sync::{Arc, LazyLock, Mutex},
};
use tracing::{error, info};
use mlua::{Lua, Result as LuaResult, Table};
use regex::Regex;

const MOD_DIR: &str = "mods";
const MAIN_FILE: &str = "main.lua";

type LuaInitFn = fn(&mut Lua) -> LuaResult<()>;
pub struct LuaRegistration(pub LuaInitFn);

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

    if let Err(e) = (|| -> LuaResult<()> {
        let globals = lua.globals();
        let package: Table = globals.get("package")?;

        package.set("path", "")?;
        package.set("cpath", "")?;

        lua.set_globals(globals)?;

        Ok(())
    })() {
        error!("Lua 初始化时出现错误");
        panic!("Lua 初始化时出现错误: {}", e);
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
                let error_msg = if let Some(extract) = EXTRACT.as_ref() {
                    extract.replace(&e.to_string(), ": ").to_string()
                } else {
                    e.to_string()
                };
                error!("{}", error_msg);
            }

            result
        },
        Err(e) => {
            error!("Lua 状态机锁错误: {}", e);
            panic!("Lua 状态机锁错误: {}", e)
        }
    }
}

pub fn load_mods() -> Result<u32> {
    let mut success = 0;
    if !Path::new(MOD_DIR).exists() {
        fs::create_dir(MOD_DIR)?;
    }

    for entry in fs::read_dir(MOD_DIR)? {
        match load_mod(entry) {
            Ok(_) => success += 1,
            Err(_) => (),
        }
    }

    Ok(success)
}

fn load_mod(entry: Result<DirEntry, std::io::Error>) -> Result<()> {
    let path = entry?.path();

    if !path.is_dir() { return Ok(()) };

    let path_str = path.to_string_lossy().to_string();

    let main_file = path.join(MAIN_FILE);

    let script = fs::read_to_string(&main_file)
        .with_context(|| format!("读取 Mod 主文件失败: {:?}", main_file))?;
    
    let mod_name = path.file_name().unwrap().to_string_lossy().to_string();

    let result = with_lua(move |lua| {

        info!("正在加载 Mod: {}", mod_name);

        // 1. 创建沙盒环境 (Sandbox Table)
        let sandbox = lua.create_table()?;

        let package: Table = lua.globals().get("package")?;

        package.set("path", format!(r"{path_str}\?.lua;{path_str}\?\init.lua"))?;
        package.set("cpath", format!(r"{path_str}\?.dll;{path_str}\?\loadall.dll"))?;

        sandbox.set("package", package)?;

        // 2. 设置元表：读取不到变量时，去全局 _G 找
        let meta = lua.create_table()?;
        meta.set("__index", lua.globals())?;
        sandbox.set_metatable(Some(meta))?;

        // 3. 执行 main.lua
        lua.load(&script)
            .set_name(format!("@{}/main.lua", mod_name))
            .set_environment(sandbox)
            .exec()?;

        Ok(())
    });

    if let Err(e) = result {
        error!("加载 Mod({}) 失败", &path.to_string_lossy());

        return Err(anyhow::anyhow!(e));
    }

    Ok(())
}
