use mlua::prelude::*;
use tracing::info;

use super::LuaRegistration;

inventory::submit! {
    LuaRegistration(|lua| {

        let globals = lua.globals();

        let log_table = lua.create_table()?;

        let info_func = lua.create_function(|_, args: LuaVariadic<LuaValue>| {
            let mut message = String::new();
            for (_, v) in args.iter().enumerate() {
                if let Ok(string) = v.to_string() {
                    message += &string;
                } else {
                    message += &format!("{:?}", v);
                }
            }

            info!("{}", message);

            Ok(())
        })?;

        log_table.set("info", info_func)?;

        globals.set("log", log_table)?;

        Ok(())
    })
}
