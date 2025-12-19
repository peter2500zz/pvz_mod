use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::trace;
use std::{collections::HashMap, sync::{LazyLock, Mutex}};

use crate::utils::data_array::HasId;

pub static PROFILE_MANAGER: LazyLock<Mutex<Profile>> = LazyLock::new(|| {
    Mutex::new(Profile::default())
});

pub const SAVES_DIR: &str = "saves/userdata";

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Profile {
    // Key 1: Namespace (e.g., "zombie", "npc", "player")
    // Key 2: Entity ID
    // Key 3: Attribute Key
    data: HashMap<String, HashMap<i32, HashMap<String, Value>>>,
}

impl Profile {
    pub fn clear(&mut self) {
        self.data.clear();
    }

    // 3. 泛型化的 Set 方法
    pub fn set_attr<T: HasId>(&mut self, entity: &T, key: String, value: LuaValue) -> LuaResult<()> {
        // 第一步：找到 Namespace 对应的 Map (如果没有则创建)
        let namespace_data = self.data.entry(T::NAMESPACE.to_string())
            .or_insert_with(HashMap::new);
        
        // 第二步：找到 ID 对应的 Map (如果没有则创建)
        let entity_data = namespace_data.entry(entity.id())
            .or_insert_with(HashMap::new);

        match serde_json::to_value(&value) {
            Ok(json_value) => {
                trace!("set val {:?} for {} {:#x}", json_value, T::NAMESPACE, entity.id());
                entity_data.insert(key, json_value);
                Ok(())
            }
            Err(e) => Err(LuaError::SerializeError(e.to_string())),
        }
    }

    // 4. 泛型化的 Get 方法
    pub fn get_attr<T: HasId>(&self, lua: &Lua, entity: &T, key: String) -> LuaValue {
        trace!("namespace level: {}", T::NAMESPACE);
        if let Some(namespace_data) = self.data.get(T::NAMESPACE) {
            trace!("entity level: {}", entity.id());
            if let Some(entity_data) = namespace_data.get(&entity.id()) {
                trace!("data level: {}", key);
                if let Some(data) = entity_data.get(&key) {
                    return lua.to_value(data).unwrap_or(LuaNil);
                }
            }
        }
        LuaNil
    }

    // 5. 泛型化的 Remove Attr 方法
    pub fn remove_attr<T: HasId>(&mut self, entity: &T, key: String) {
        if let Some(namespace_data) = self.data.get_mut(T::NAMESPACE) {
            if let Some(entity_data) = namespace_data.get_mut(&entity.id()) {
                entity_data.remove(&key);
            }
        }
    }

    // 6. 泛型化的 Remove Entity 方法 (删除整个实体的数据)
    pub fn remove_entity<T: HasId>(&mut self, entity: &T) {
        if let Some(namespace_data) = self.data.get_mut(T::NAMESPACE) {
            namespace_data.remove(&entity.id());
        }
    }
}
