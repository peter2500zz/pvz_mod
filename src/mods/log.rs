use mlua::prelude::*;
use tracing::{
    trace,
    debug,
    info,
    warn,
    error
};

use super::LuaRegistration;

macro_rules! create_log_func {
    ($lua:expr, $log_table:expr, $name:expr, $macro:ident) => {{
        let func = $lua.create_function(|_, args: LuaVariadic<LuaValue>| {
            let message: String = args
                .iter()
                .map(|v| v.to_string().unwrap_or_else(|_| format!("{:?}", v)))
                .collect();
            
            $macro!("{}", message);
            Ok(())
        })?;
        $log_table.set($name, func)?;
    }};
}


inventory::submit! {
    LuaRegistration(|lua| {

        let globals = lua.globals();

        let log_table = lua.create_table()?;

        create_log_func!(lua, log_table, "trace", trace);
        create_log_func!(lua, log_table, "debug", debug);
        create_log_func!(lua, log_table, "info", info);
        create_log_func!(lua, log_table, "warn", warn);
        create_log_func!(lua, log_table, "error", error);

        globals.set("Log", log_table)?;

        Ok(())
    })
}
