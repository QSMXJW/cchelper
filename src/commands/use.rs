use crate::config::{load_config, resolve_model_name, save_config};
use crate::error::{CchelperError, Result};

pub fn cmd_use(model_name: &str) -> Result<()> {
    let mut config = load_config()?;

    let real_model = resolve_model_name(&config, model_name)
        .ok_or_else(|| CchelperError::ModelNotFound(model_name.to_string()))?;

    config.current_model = Some(real_model.clone());
    save_config(&config)?;

    if model_name != real_model {
        eprintln!("\n✅ 已切换到模型：\x1b[32m{}\x1b[0m (别名：{})\n", real_model, model_name);
    } else {
        eprintln!("\n✅ 已切换到模型：\x1b[32m{}\x1b[0m\n", real_model);
    }

    Ok(())
}
