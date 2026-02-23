use crate::config::{load_config, save_config};
use crate::error::Result;
use crate::config::ModelConfig;
use crate::style::{GREEN, RESET};

pub fn cmd_add(name: &str, api_key: &str, base_url: &str) -> Result<()> {
    let mut config = load_config()?;

    config.models.insert(
        name.to_string(),
        ModelConfig {
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
            aliases: vec![],
        },
    );

    save_config(&config)?;
    eprintln!("{}✅ 已添加模型配置：{}{}{}\n", GREEN, RESET, GREEN, name);

    Ok(())
}
