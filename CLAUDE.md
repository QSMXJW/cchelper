# Claude Code 项目配置

## 项目概述

cchelper 是一个 Claude Code 模型配置切换工具，用于管理多个 Claude API 模型配置并快速切换。

## 技术栈

- 语言：Rust
- 构建工具：Cargo

## 代码规范

### 格式化

- 使用 `cargo fmt` 格式化代码
- 遵循 Rust 默认风格指南

### 注释

- 公共函数必须添加文档注释 `///`
- 复杂逻辑添加行内注释说明意图

### 错误处理

- 使用 `thiserror` 定义错误类型
- 返回 `Result<T, CchelperError>` 类型

## 命令行参数

- 使用 `clap` 库解析命令行参数
- 支持子命令和别名（如 `s` 是 `start` 的别名）

## 测试

- 核心逻辑必须包含单元测试
- 使用 `#[test]` 属性标记测试
- 测试文件放在对应模块的 `mod tests` 中
