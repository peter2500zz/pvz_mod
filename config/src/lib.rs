use serde::{Serialize, de::DeserializeOwned};
use serde_yaml::Value;
use std::fs;
use windows_wrapper::mb;

/// 从文件中读取结构体需要的内容
pub fn load_config<T: DeserializeOwned>(path: &str) -> T {
    match serde_yaml::from_str::<T>(&fs::read_to_string(path).unwrap_or_default())
    {
        Ok(cfg) => cfg,
        Err(e) => {
            mb!("读取配置文件时出现错误\n{}: {}", path, e);
            panic!("{}", e)
        }
    }
}

/// 保存结构体但不修改其他内容
pub fn save_config<T: Serialize + DeserializeOwned>(path: &str, config: &T) {
    let existing_content = fs::read_to_string(path).unwrap_or_default();
    
    let mut yaml_value: Value = serde_yaml::from_str(&existing_content)
        .unwrap_or(Value::Mapping(Default::default()));
    
    let config_value = match serde_yaml::to_value(config) {
        Ok(v) => v,
        Err(e) => {
            mb!("保存配置文件时出现错误\n{}: {}", path, e);
            panic!("{}", e)
        }
    };
    
    if let (Value::Mapping(map), Value::Mapping(config_map)) = (&mut yaml_value, config_value.clone()) {
        for (k, v) in config_map {
            map.insert(k, v);
        }
    } else {
        yaml_value = config_value;
    }
    
    match fs::write(path, serde_yaml::to_string(&yaml_value).unwrap()) {
        Ok(_) => {},
        Err(e) => {
            mb!("保存配置文件时出现错误\n{}: {}", path, e);
            panic!("{}", e)
        }
    }
}
