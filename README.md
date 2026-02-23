# cchelper

Claude Code 模型配置切换工具。

## 功能特性

- **多模型管理**：管理多个 Claude API 模型配置
- **快速切换**：在不同的模型配置之间快速切换
- **别名支持**：为模型设置简短的别名方便切换
- **一键启动**：使用当前配置的模型启动 Claude 会话

## 安装

```bash
cargo build --release
```

将生成的二进制文件添加到 PATH 中：

```bash
cp target/release/cchelper /usr/local/bin/
```

## 使用方法

### 初始化配置

```bash
cchelper init
```

### 添加模型配置

```bash
cchelper add <model-name> --api-key <your-api-key> --base-url <optional-base-url>
```

示例：

```bash
cchelper add claude-opus --api-key sk-xxx
```

### 列出所有模型

```bash
cchelper list
```

### 切换模型

```bash
cchelper use <model-name-or-alias>
```

### 删除模型

```bash
cchelper delete <model-name>
```

### 显示当前配置

```bash
cchelper config
```

### 启动 Claude 会话

使用当前配置的模型启动 Claude：

```bash
cchelper start
```

### 别名管理

添加别名：

```bash
cchelper alias add <model-name> <alias>
```

删除别名：

```bash
cchelper alias remove <model-name> <alias>
```

列出所有别名：

```bash
cchelper alias list
```

## 配置文件

配置文件保存在 `~/.cchelper/config.json`，格式如下：

```json
{
  "models": {
    "claude-opus": {
      "api_key": "sk-xxx",
      "base_url": "",
      "aliases": ["opus"]
    }
  },
  "current_model": "claude-opus"
}
```

## 命令列表

| 命令 | 说明 |
|------|------|
| `cchelper init` | 初始化配置文件 |
| `cchelper add <name>` | 添加新模型配置 |
| `cchelper list` | 列出所有可用模型 |
| `cchelper use <name>` | 切换到指定模型 |
| `cchelper delete <name>` | 删除模型配置 |
| `cchelper config` | 显示当前配置 |
| `cchelper start` | 启动 Claude 会话 |
| `cchelper alias add <model> <alias>` | 添加别名 |
| `cchelper alias remove <model> <alias>` | 删除别名 |
| `cchelper alias list` | 列出所有别名 |

## 环境要求

- Rust 2021 Edition
- Claude Code CLI

## License

MIT
