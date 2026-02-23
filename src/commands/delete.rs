use crate::config::{load_config, save_config};
use crate::error::{CchelperError, Result};
use crate::style::{RED, RESET};

pub fn cmd_delete(model_name: &str) -> Result<()> {
    let mut config = load_config()?;

    if !config.models.contains_key(model_name) {
        return Err(CchelperError::ModelNotFound(model_name.to_string()));
    }

    config.models.remove(model_name);

    if config.current_model.as_deref() == Some(model_name) {
        config.current_model = None;
    }

    save_config(&config)?;
    eprintln!("{}❌ 已删除模型配置：{}{}{}\n", RED, RED, model_name, RESET);

    Ok(())
}
