use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::trace;
use std::{collections::HashMap, sync::{LazyLock, Mutex}};

use crate::pvz::board::board::Board;

pub static PROFILE_MANAGER: LazyLock<Mutex<Profile>> = LazyLock::new(|| {
    Mutex::new(Profile::default())
});

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Profile {
    zombies: HashMap<i32, HashMap<String, Value>>,
}

impl Profile {
    pub fn fix_board(&self, board: *mut Board) {
        // 还原僵尸数据
    }

    pub fn set_zombie_attr(&mut self, id: i32, key: String, value: LuaValue) -> LuaResult<()> {
        let zombie_data = self.zombies.entry(id).or_insert(HashMap::new());

        match serde_json::to_value(value) {
            Ok(json_value) => {
                trace!("set val {:?} for zombie {:#x}", json_value, id);
                (*zombie_data).insert(key, json_value);
                Ok(())
            }
            Err(e) => Err(LuaError::SerializeError(e.to_string())),
        }
    }

    pub fn get_zombie_attr(&self, lua: &Lua, id: i32, key: String) -> LuaValue {
        if let Some(zombie_data) = self.zombies.get(&id) 
        && let Some(data) = zombie_data.get(&key)
        {
            lua.to_value(data).unwrap_or(LuaNil)
        } else {
            LuaNil
        }
    }

    pub fn remove_zombie_attr(&mut self, id: i32, key: String) {
        if let Some(zombie_data) = self.zombies.get_mut(&id) 
        {
            zombie_data.remove(&key);
        }
    }
}
