use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::error::Result;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ModelConfig {
    pub api_key: String,
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub aliases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub models: HashMap<String, ModelConfig>,
    pub current_model: Option<String>,
}

fn get_config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("无法获取家目录")
        .join(".cchelper")
}

pub fn get_config_path() -> PathBuf {
    get_config_dir().join("config.json")
}

pub fn ensure_config_dir() -> Result<()> {
    let config_dir = get_config_dir();
    fs::create_dir_all(&config_dir)?;
    Ok(())
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path();
    if !config_path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(&config_path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    ensure_config_dir()?;
    let config_path = get_config_path();
    let content = serde_json::to_string_pretty(config)?;
    fs::write(&config_path, content)?;
    Ok(())
}

/// 根据模型名称或别名查找真实模型名
pub fn resolve_model_name(config: &Config, name: &str) -> Option<String> {
    // 首先检查是否是真实模型名
    if config.models.contains_key(name) {
        return Some(name.to_string());
    }

    // 然后检查是否是别名
    for (model_name, model_config) in &config.models {
        if model_config.aliases.iter().any(|a| a == name) {
            return Some(model_name.clone());
        }
    }

    None
}
