use crate::config::{ensure_config_dir, get_config_path, save_config, Config};
use crate::error::Result;
use crate::style::{GREEN, RESET};

pub fn cmd_init() -> Result<()> {
    ensure_config_dir()?;

    let config_path = get_config_path();
    if config_path.exists() {
        eprintln!("{}⚠️  配置文件已存在：{}{}\n", GREEN, config_path.display(), RESET);
        return Ok(());
    }

    let config = Config::default();
    save_config(&config)?;
    eprintln!("{}✅ 已初始化配置文件：{}{}\n", GREEN, config_path.display(), RESET);

    Ok(())
}
