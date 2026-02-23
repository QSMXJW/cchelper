use crate::config::{load_config, save_config};
use crate::error::{CchelperError, Result};
use crate::style::{GREEN, RED, RESET, YELLOW, CYAN};

pub fn cmd_alias_add(model: &str, alias: &str) -> Result<()> {
    let mut config = load_config()?;

    // 检查模型是否存在
    if !config.models.contains_key(model) {
        return Err(CchelperError::ModelNotFound(model.to_string()));
    }

    // 检查别名是否是其他模型的别名
    for (model_name, mc) in &config.models {
        if model_name != model && mc.aliases.iter().any(|a| a == alias) {
            return Err(CchelperError::AliasExists(model_name.clone(), alias.to_string()));
        }
    }

    let model_config = config.models.get_mut(model).unwrap();

    // 检查别名是否已存在
    if model_config.aliases.iter().any(|a| a == alias) {
        return Err(CchelperError::AliasExists(model.to_string(), alias.to_string()));
    }

    model_config.aliases.push(alias.to_string());
    save_config(&config)?;
    eprintln!("{}✅ 已为模型 '{}{}{}{}' 添加别名：'{}{}{}{}'\n", GREEN, RESET, CYAN, model, GREEN, RESET, YELLOW, alias, RESET);

    Ok(())
}

pub fn cmd_alias_remove(model: &str, alias: &str) -> Result<()> {
    let mut config = load_config()?;

    let model_config = config
        .models
        .get_mut(model)
        .ok_or_else(|| CchelperError::ModelNotFound(model.to_string()))?;

    let pos = model_config
        .aliases
        .iter()
        .position(|a| a == alias)
        .ok_or_else(|| CchelperError::AliasNotFound(alias.to_string()))?;

    model_config.aliases.remove(pos);
    save_config(&config)?;
    eprintln!("{}❌ 已删除模型 '{}{}{}{}' 的别名：'{}{}{}{}'\n", RED, RESET, CYAN, model, RED, RESET, YELLOW, alias, RESET);

    Ok(())
}

pub fn cmd_alias_list() -> Result<()> {
    let config = load_config()?;

    let mut has_aliases = false;
    println!("{}🏷️  模型别名：{}\n", CYAN, RESET);
    for (name, model_config) in &config.models {
        if !model_config.aliases.is_empty() {
            has_aliases = true;
            println!("  {}{}{} => {}{}{}", CYAN, name, RESET, YELLOW, model_config.aliases.join(", "), RESET);
        }
    }

    if !has_aliases {
        println!("{}⚠️  暂无别名配置{}\n", YELLOW, RESET);
        println!("  使用 'cchelper alias add <model> <alias>' 添加别名");
    }

    Ok(())
}
