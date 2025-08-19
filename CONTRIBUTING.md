# 贡献指南 (Contributing Guide)

感谢您对 Rust 训练营项目的关注！我们欢迎所有形式的贡献，包括但不限于代码改进、文档完善、问题报告和功能建议。

## 📋 目录

- [行为准则](#行为准则)
- [如何贡献](#如何贡献)
- [开发环境设置](#开发环境设置)
- [代码规范](#代码规范)
- [提交规范](#提交规范)
- [Pull Request 流程](#pull-request-流程)
- [问题报告](#问题报告)
- [功能请求](#功能请求)

## 🤝 行为准则

参与本项目即表示您同意遵守我们的行为准则：

- 使用友好和包容的语言
- 尊重不同的观点和经验
- 优雅地接受建设性批评
- 专注于对社区最有利的事情
- 对其他社区成员表现出同理心

## 🚀 如何贡献

### 贡献类型

1. **代码贡献**
   - Bug 修复
   - 新功能实现
   - 性能优化
   - 代码重构

2. **文档贡献**
   - README 改进
   - 代码注释完善
   - 教程和示例
   - API 文档

3. **测试贡献**
   - 单元测试
   - 集成测试
   - 性能测试
   - 测试覆盖率提升

4. **其他贡献**
   - 问题报告
   - 功能建议
   - 代码审查
   - 社区支持

## 🛠️ 开发环境设置

### 前置要求

- **Rust**: 1.70.0 或更高版本
- **Git**: 最新稳定版本
- **PostgreSQL**: 13+ (仅 rust-chat 项目需要)
- **IDE**: 推荐使用 VS Code + rust-analyzer

### 环境配置

1. **Fork 并克隆仓库**
   ```bash
   git clone https://github.com/YOUR_USERNAME/rust-bootcamp.git
   cd rust-bootcamp
   ```

2. **安装 Rust 工具链**
   ```bash
   # 安装 rustup (如果尚未安装)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # 安装必要组件
   rustup component add rustfmt clippy
   ```

3. **设置开发工具**
   ```bash
   # 安装 cargo-watch (可选，用于自动重新编译)
   cargo install cargo-watch
   
   # 安装 sqlx-cli (仅 rust-chat 项目需要)
   cargo install sqlx-cli --no-default-features --features postgres
   ```

4. **验证环境**
   ```bash
   # 检查 Rust 版本
   rustc --version
   cargo --version
   
   # 运行测试
   cargo test
   ```

## 📝 代码规范

### Rust 代码风格

1. **格式化**
   ```bash
   # 使用 rustfmt 格式化代码
   cargo fmt
   ```

2. **Linting**
   ```bash
   # 使用 clippy 检查代码质量
   cargo clippy -- -D warnings
   ```

3. **命名约定**
   - 使用 `snake_case` 命名变量和函数
   - 使用 `PascalCase` 命名类型和 trait
   - 使用 `SCREAMING_SNAKE_CASE` 命名常量
   - 使用 `kebab-case` 命名包和二进制文件

4. **代码组织**
   ```rust
   // 导入顺序：标准库 -> 第三方库 -> 本地模块
   use std::collections::HashMap;
   
   use serde::{Deserialize, Serialize};
   use tokio::net::TcpListener;
   
   use crate::error::AppError;
   use crate::models::User;
   ```

5. **错误处理**
   ```rust
   // 优先使用 Result 类型
   fn process_data(input: &str) -> Result<String, AppError> {
       // 使用 ? 操作符传播错误
       let processed = validate_input(input)?;
       Ok(processed.to_uppercase())
   }
   ```

6. **文档注释**
   ```rust
   /// 创建新用户
   /// 
   /// # Arguments
   /// 
   /// * `email` - 用户邮箱地址
   /// * `password` - 用户密码
   /// 
   /// # Returns
   /// 
   /// 返回创建的用户信息或错误
   /// 
   /// # Examples
   /// 
   /// ```
   /// let user = create_user("test@example.com", "password123")?;
   /// ```
   pub async fn create_user(email: &str, password: &str) -> Result<User, AppError> {
       // 实现代码
   }
   ```

### 测试规范

1. **单元测试**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
   
       #[test]
       fn test_user_creation() {
           // 测试代码
       }
   
       #[tokio::test]
       async fn test_async_function() {
           // 异步测试代码
       }
   }
   ```

2. **集成测试**
   - 将集成测试放在 `tests/` 目录下
   - 每个测试文件应该测试一个特定的功能模块

## 📤 提交规范

### Commit Message 格式

使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### 提交类型

- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式化（不影响功能）
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动
- `perf`: 性能优化
- `ci`: CI/CD 相关

### 示例

```bash
# 好的提交消息
feat(chat): add user authentication system
fix(cli): resolve file parsing error
docs: update installation guide
test(user): add unit tests for user model

# 不好的提交消息
update code
fix bug
add stuff
```

## 🔄 Pull Request 流程

### 提交 PR 前的检查清单

- [ ] 代码已通过 `cargo fmt` 格式化
- [ ] 代码已通过 `cargo clippy` 检查
- [ ] 所有测试都通过 (`cargo test`)
- [ ] 添加了必要的测试用例
- [ ] 更新了相关文档
- [ ] 提交消息符合规范

### PR 模板

创建 PR 时，请包含以下信息：

```markdown
## 变更描述

简要描述此 PR 的变更内容。

## 变更类型

- [ ] Bug 修复
- [ ] 新功能
- [ ] 文档更新
- [ ] 代码重构
- [ ] 性能优化
- [ ] 其他（请说明）

## 测试

描述如何测试这些变更：

- [ ] 单元测试
- [ ] 集成测试
- [ ] 手动测试

## 检查清单

- [ ] 代码已格式化
- [ ] 通过 clippy 检查
- [ ] 所有测试通过
- [ ] 文档已更新
```

### 代码审查

1. **审查重点**
   - 代码正确性和逻辑
   - 性能和安全性
   - 代码可读性和维护性
   - 测试覆盖率

2. **反馈处理**
   - 及时回应审查意见
   - 进行必要的修改
   - 解释设计决策

## 🐛 问题报告

### 报告 Bug

使用 GitHub Issues 报告问题，请包含：

1. **环境信息**
   - 操作系统和版本
   - Rust 版本 (`rustc --version`)
   - 项目版本或 commit hash

2. **问题描述**
   - 预期行为
   - 实际行为
   - 重现步骤

3. **附加信息**
   - 错误日志
   - 相关代码片段
   - 截图（如适用）

### Bug 报告模板

```markdown
**环境信息**
- OS: [e.g. Windows 11, macOS 13, Ubuntu 22.04]
- Rust版本: [e.g. 1.70.0]
- 项目版本: [e.g. commit hash]

**问题描述**
简要描述遇到的问题。

**重现步骤**
1. 执行 '...'
2. 点击 '....'
3. 滚动到 '....'
4. 看到错误

**预期行为**
描述您期望发生的情况。

**实际行为**
描述实际发生的情况。

**错误日志**
```
粘贴相关的错误日志
```

**附加信息**
添加任何其他有助于解决问题的信息。
```

## 💡 功能请求

### 提交功能请求

1. **搜索现有 Issues**
   - 确保功能尚未被请求
   - 查看是否有相关讨论

2. **描述功能**
   - 清晰描述所需功能
   - 解释使用场景
   - 提供实现建议（可选）

3. **考虑影响**
   - 对现有功能的影响
   - 性能考虑
   - 维护成本

## 🏷️ 版本发布

### 语义化版本

我们遵循 [Semantic Versioning](https://semver.org/) 规范：

- `MAJOR`: 不兼容的 API 变更
- `MINOR`: 向后兼容的功能新增
- `PATCH`: 向后兼容的问题修正

### 发布流程

1. 更新版本号
2. 更新 CHANGELOG.md
3. 创建 Git tag
4. 发布到 crates.io（如适用）

## 📞 联系方式

如有任何问题或建议，请通过以下方式联系：

- GitHub Issues: 项目相关问题
- GitHub Discussions: 一般讨论和问答
- Email: [维护者邮箱]

## 🙏 致谢

感谢所有为本项目做出贡献的开发者！您的贡献让这个项目变得更好。

---

再次感谢您的贡献！🎉