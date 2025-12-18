use serde::de::DeserializeOwned;
use std::fs;
use windows_wrapper::mb;

pub fn load_config<T: DeserializeOwned>(path: &str) -> T {
    match fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .and_then(|text| serde_yaml::from_str::<T>(&text).map_err(|e| e.to_string()))
    {
        Ok(cfg) => cfg,
        Err(e) => {
            mb!("读取配置文件时出现错误\n{}: {}", path, e);
            panic!("{}", e)
        }
    }
}
