use clap::{Parser, Subcommand};
use cchelper::commands::{
    r#use as use_cmd,
    start, list, add, delete, config, init, alias,
};

#[derive(Parser)]
#[command(name = "cchelper")]
#[command(about = "Claude Code 模型配置切换工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 切换到指定模型（支持别名）
    Use {
        /// 模型名称或别名
        model_name: String,
    },
    /// 列出所有可用模型
    List,
    /// 添加新模型配置
    Add {
        /// 模型名称
        name: String,
        /// API Key
        #[arg(long)]
        api_key: String,
        /// Base URL (可选)
        #[arg(long, default_value = "")]
        base_url: String,
    },
    /// 删除模型配置
    Delete {
        /// 模型名称
        model_name: String,
    },
    /// 显示当前配置
    Config,
    /// 初始化配置文件
    Init,
    /// 启动 Claude 会话（使用当前配置的模型）
    Start,
    /// 别名管理
    #[command(subcommand)]
    Alias(AliasCommands),
}

#[derive(Subcommand)]
enum AliasCommands {
    /// 添加别名
    Add {
        /// 模型名称
        model: String,
        /// 别名
        alias: String,
    },
    /// 删除别名
    Remove {
        /// 模型名称
        model: String,
        /// 别名
        alias: String,
    },
    /// 列出所有别名
    List,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Use { model_name } => use_cmd::cmd_use(&model_name),
        Commands::List => list::cmd_list(),
        Commands::Add { name, api_key, base_url } => add::cmd_add(&name, &api_key, &base_url),
        Commands::Delete { model_name } => delete::cmd_delete(&model_name),
        Commands::Config => config::cmd_config(),
        Commands::Init => init::cmd_init(),
        Commands::Start => start::cmd_start(),
        Commands::Alias(AliasCommands::Add { model, alias }) => alias::cmd_alias_add(&model, &alias),
        Commands::Alias(AliasCommands::Remove { model, alias }) => alias::cmd_alias_remove(&model, &alias),
        Commands::Alias(AliasCommands::List) => alias::cmd_alias_list(),
    };

    if let Err(e) = result {
        eprintln!("错误：{}", e);
        std::process::exit(1);
    }
}
