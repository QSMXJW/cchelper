use thiserror::Error;

#[derive(Error, Debug)]
pub enum CchelperError {
    #[error("配置目录创建失败：{0}")]
    DirCreateError(#[from] std::io::Error),

    #[error("配置文件读取失败：{0}")]
    ConfigReadError(#[from] serde_json::Error),

    #[error("模型 '{0}' 不存在")]
    ModelNotFound(String),

    #[error("模型 '{0}' 已有别名 '{1}'")]
    AliasExists(String, String),

    #[error("别名 '{0}' 不存在")]
    AliasNotFound(String),
}

pub type Result<T> = std::result::Result<T, CchelperError>;
