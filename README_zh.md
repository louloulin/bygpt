# 基于 Rust 和 WebAssembly 的智能体

本项目使用 Rust 和 WebAssembly (Wasm) 实现了一个智能体。该智能体旨在观察其环境、思考并做出决策、根据决策采取行动，并记住过去的经验。项目被模块化为多个组件，每个组件负责特定的功能。

## 功能

- **观察**：处理输入数据以理解环境。
- **思考**：使用策略根据观察结果做出决策。
- **行动**：根据决策执行行动。
- **记忆**：存储过去的观察和决策以供将来参考。
- **WebAssembly**：将 Rust 代码编译为 WebAssembly 以用于基于 Web 的应用程序。

## 安装

### 前提条件

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### 步骤

1. 克隆仓库：
   ```bash
   git clone https://github.com/yourusername/rust-wasm-agent.git
   cd rust-wasm-agent
   ```

2. 构建项目：
   ```bash
   cargo build --release
   ```

3. 构建 WebAssembly 包：
   ```bash
   wasm-pack build
   ```

## 使用

### 运行智能体

要在本地运行智能体，请使用以下命令：

```bash
cargo run
```

### WebAssembly 集成

要将 WebAssembly 模块集成到 Web 应用程序中，请按照以下步骤操作：

1. 使用静态文件服务器提供 `wasm-pack` 生成的 `pkg` 目录。
2. 在 HTML 文件中包含生成的 JavaScript 绑定。
3. 从 JavaScript 代码中调用导出的函数。

## 项目结构

```
src/
├── main.rs                # 应用程序的入口点
├── llm/                   # 语言模型模块
│   ├── mod.rs
│   └── client.rs
├── observation/           # 观察模块
│   ├── mod.rs
│   ├── data.rs
│   └── processor.rs
├── thinking/              # 思考模块
│   ├── mod.rs
│   ├── decision.rs
│   └── strategy.rs
├── action/                # 行动模块
│   ├── mod.rs
│   ├── executor.rs
│   └── types.rs
└── memory/                # 记忆模块
    ├── mod.rs
    ├── storage.rs
    └── types.rs
```

## 贡献

欢迎贡献！请 fork 仓库并提交 pull request 以进行任何改进或错误修复。

## 许可证

本项目采用 MIT 许可证。有关详细信息，请参阅 [LICENSE](LICENSE) 文件。 