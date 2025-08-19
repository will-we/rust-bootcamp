# Rust 训练营 (Rust Bootcamp)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

一个全面的 Rust 学习训练营项目，涵盖从基础 CLI 工具开发到高级 Web 应用构建的完整学习路径。

## 📚 项目概述

本训练营通过四个递进的实战项目，帮助开发者系统性地掌握 Rust 编程语言及其生态系统：

### 🛠️ [01-rust-cli](01-rust-cli) - CLI 工具开发
- **目标**: 掌握 Rust 基础语法和 CLI 应用开发
- **技术栈**: `clap`, `serde`, `tokio`
- **功能**: 命令行参数解析、文件处理、数据序列化
- **学习重点**: 所有权系统、错误处理、模块化设计

### 🔧 [03-macros](03-macros) - 宏编程
- **目标**: 深入理解 Rust 宏系统
- **技术栈**: 声明式宏、过程宏
- **功能**: 自定义宏实现、代码生成
- **学习重点**: 元编程、编译时代码生成、宏展开机制

### 🌐 [04-rust-ecosystem](04-rust-ecosystem) - 生态系统探索
- **目标**: 熟悉 Rust 生态系统核心库
- **技术栈**: `axum`, `serde`, `tracing`, `anyhow`
- **功能**: Web 服务开发、序列化、日志记录、错误处理
- **学习重点**: 异步编程、Web 框架、中间件、依赖注入

### 💬 [05-rust-chat](05-rust-chat) - 实时聊天应用
- **目标**: 构建完整的现代化 Web 应用
- **技术栈**: `axum`, `sqlx`, `postgresql`, `tokio`, `tracing`
- **架构**: 微服务架构 (chat_server + notify_server)
- **功能**: 
  - 用户认证与授权
  - 实时消息传递
  - 数据库持久化
  - Server-Sent Events (SSE)
  - RESTful API 设计
- **学习重点**: 数据库集成、实时通信、微服务架构、生产级应用开发

## 🚀 快速开始

### 环境要求
- Rust 1.70+
- PostgreSQL 13+ (仅 rust-chat 项目需要)
- Git

### 安装步骤

1. **克隆项目**
   ```bash
   git clone <repository-url>
   cd rust-bootcamp
   ```

2. **按顺序学习各个项目**
   ```bash
   # CLI 工具开发
   cd 01-rust-cli
   cargo run
   
   # 宏编程
   cd ../03-macros
   cargo run
   
   # 生态系统探索
   cd ../04-rust-ecosystem
   cargo run --example axum_serde
   
   # 聊天应用 (需要先配置数据库)
   cd ../05-rust-chat
   # 配置 .env 文件
   cargo run --bin chat-server
   ```

3. **聊天应用额外配置**
   ```bash
   cd 05-rust-chat
   
   # 创建 .env 文件
   echo "DATABASE_URL=postgres://postgres:password@localhost:5432/chat" > .env
   
   # 运行数据库迁移
   sqlx migrate run
   
   # 启动聊天服务器
   cargo run --bin chat-server
   
   # 启动通知服务器 (另一个终端)
   cargo run --bin notify-server
   ```

## 🎯 学习路径

### 初级 (Week 1-2)
- 完成 `01-rust-cli` 项目
- 掌握 Rust 基础语法、所有权系统
- 学习错误处理和模块化设计

### 中级 (Week 3-4)
- 完成 `03-macros` 和 `04-rust-ecosystem` 项目
- 深入理解宏编程和异步编程
- 熟悉 Rust 生态系统核心库

### 高级 (Week 5-8)
- 完成 `05-rust-chat` 项目
- 掌握数据库集成、微服务架构
- 学习生产级应用开发最佳实践

## 🛡️ 技术特色

- **内存安全**: 零成本抽象，无垃圾回收的内存管理
- **并发安全**: 编译时防止数据竞争
- **高性能**: 接近 C/C++ 的运行时性能
- **现代化**: 优秀的包管理器和工具链
- **生产就绪**: 企业级应用开发能力

## 📖 学习资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/)
- [Rust 异步编程](https://rust-lang.github.io/async-book/)
- [Axum 框架文档](https://docs.rs/axum/latest/axum/)

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详细信息。

## 📄 许可证

本项目采用双许可证：

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

您可以选择其中任一许可证使用本项目。