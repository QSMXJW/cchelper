use std::os::unix::process::CommandExt;
use std::process::Command;

use crate::config::load_config;
use crate::error::{CchelperError, Result};

pub fn cmd_start() -> Result<()> {
    let config = load_config()?;

    let current_model = config.current_model.ok_or_else(|| {
        CchelperError::ModelNotFound("当前未选择任何模型，请先使用 'cchelper use <model>' 切换".to_string())
    })?;

    let model_config = config.models.get(&current_model).unwrap();

    // 构建环境变量
    let mut envs = vec![
        ("ANTHROPIC_API_KEY", &model_config.api_key),
        ("ANTHROPIC_MODEL", &current_model),
    ];

    if !model_config.base_url.is_empty() {
        envs.push(("ANTHROPIC_BASE_URL", &model_config.base_url));
    }

    eprintln!("\n🚀 启动 Claude 会话，使用模型：\x1b[32m{}\x1b[0m\n", current_model);

    // 使用 exec 替换当前进程，这样退出 claude 后不会返回 cchelper
    let err = Command::new("claude")
        .envs(envs)
        .arg("--model")
        .arg(&current_model)
        .exec();

    // 只有 exec 失败时才会执行到这里
    Err(CchelperError::DirCreateError(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("启动 claude 失败：{}", err),
    )))
}
