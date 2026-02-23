use crate::config::load_config;
use crate::error::Result;
use crate::style::{BOLD, CYAN, GREEN, RESET, YELLOW};

pub fn cmd_list() -> Result<()> {
    let config = load_config()?;

    if config.models.is_empty() {
        println!("{}⚠️  暂无配置的模型，使用 'cchelper add <name> --api-key <key>' 添加{}\n", YELLOW, RESET);
        return Ok(());
    }

    println!("{}📦 已配置的模型:{}\n", BOLD, RESET);
    for (name, model_config) in &config.models {
        let current_marker = if config.current_model.as_deref() == Some(name) {
            format!(" {}(当前){}", GREEN, RESET)
        } else {
            "".to_string()
        };

        if model_config.aliases.is_empty() {
            println!("  {}- {}{}{}", CYAN, name, RESET, current_marker);
        } else {
            println!("  {}- {} {} [别名：{}{}]{}", CYAN, name, YELLOW, model_config.aliases.join(", "), RESET, current_marker);
        }
    }

    Ok(())
}
