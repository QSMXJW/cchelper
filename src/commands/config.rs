use crate::config::load_config;
use crate::error::Result;
use crate::style::{BOLD, CYAN, GREEN, RESET, YELLOW};

pub fn cmd_config() -> Result<()> {
    let config = load_config()?;

    match &config.current_model {
        Some(model) => {
            println!("{}📌 当前模型：{}{}{}", BOLD, GREEN, model, RESET);
            if let Some(model_config) = config.models.get(model) {
                let base_url = if model_config.base_url.is_empty() {
                    "default".to_string()
                } else {
                    model_config.base_url.clone()
                };
                println!("{}Base URL:{} {}", YELLOW, RESET, base_url);
                if !model_config.aliases.is_empty() {
                    println!("{}别名：{} {}", CYAN, RESET, model_config.aliases.join(", "));
                }
            }
        }
        None => {
            println!("{}⚠️  当前未选择任何模型{}\n", YELLOW, RESET);
        }
    }

    println!("\n{}📦 所有可用模型：{}\n", BOLD, RESET);
    for (name, model_config) in config.models.iter() {
        if model_config.aliases.is_empty() {
            println!("  {}- {}{}", CYAN, name, RESET);
        } else {
            println!("  {}- {} {}[别名：{}{}]{}", CYAN, name, RESET, YELLOW, model_config.aliases.join(", "), RESET);
        }
    }

    Ok(())
}
