# 参与 PalaceMail 开发

感谢你的兴趣。本文说明仓库的目录结构、如何在本地运行，以及提交规范。设计上的"为什么"请读 [docs/architecture.md](docs/architecture.md)。

## 目录结构

```
palacemail/
├── apps/
│   └── desktop/            Tauri 2 桌面应用
│       ├── src/            React + TypeScript 界面（Vite）
│       └── src-tauri/      Rust 壳：创建窗口、注册 IPC 命令，只依赖 palace-core
├── crates/
│   ├── core/               palace-core：内核对外 API，桌面壳与未来的 CLI、守护进程都只经由它访问内核
│   ├── assistant/          palace-assistant：供应商抽象、提示词注册表、上下文组装、AI 流水线
│   ├── store/              palace-store：唯一事实来源——规范化消息、会话、FTS、向量、派生数据、作业队列
│   └── mail/               palace-mail：IMAP/SMTP 适配、OAuth、增量同步、出站队列
├── packages/
│   └── ui/                 @palacemail/ui：界面组件与设计令牌（tokens/）
├── docs/                   设计文档
├── .github/workflows/      CI
├── Cargo.toml              Rust 工作区
└── package.json            pnpm 工作区
```

四个 crate 之间的依赖是严格单向的：

```
core → assistant → store → mail
```

上层只能依赖紧邻的下层，不得反向引用；`ui` 不直接调用模型，`assistant` 不碰 IMAP，`mail` 不知道 AI 的存在。新增依赖前先确认它不会破坏这条链。

## 本地运行

### 前置条件

- Rust 稳定版：通过 [rustup](https://rustup.rs) 安装；仓库内的 `rust-toolchain.toml` 会自动选用 stable 并安装 rustfmt 与 clippy。
- Tauri 2 的系统依赖：按平台见 [Tauri 官方文档](https://tauri.app/start/prerequisites/)。Windows 需要 WebView2 与 MSVC 生成工具，Linux 需要 webkit2gtk 4.1 等开发包，macOS 需要 Xcode 命令行工具。
- Node.js ≥ 22.12 与 pnpm 11：执行 `corepack enable` 后会自动匹配 `package.json` 里的 `packageManager` 版本。

### 常用命令

前端与桌面应用：

```bash
pnpm install                 # 安装工作区依赖
pnpm tauri dev               # 启动桌面应用：同时拉起 Vite 开发服务器与 cargo
pnpm --filter desktop dev    # 只启动前端，在浏览器中打开（此时 IPC 不可用）
pnpm --filter desktop build  # 类型检查 + 前端构建
pnpm lint                    # oxlint
pnpm tauri build             # 产出当前平台的安装包
```

Rust 工作区：

```bash
cargo check --workspace                                  # 快速编译检查
cargo fmt --all                                          # 格式化
cargo clippy --workspace --all-targets -- -D warnings    # 静态检查，CI 使用同样的参数
cargo test --workspace                                   # 单元测试
```

提交前请至少跑通 `cargo fmt --all --check`、上面的 clippy 与 test，以及 `pnpm lint` 与 `pnpm --filter desktop build`；这也正是 CI 的全部内容。

### 重新生成应用图标

图标源文件是 `apps/desktop/app-icon.svg`，修改后在 `apps/desktop` 下运行 `pnpm tauri icon app-icon.svg` 即可重新生成 `src-tauri/icons/`。

## 提交规范

- **语言**：提交信息、PR 标题与描述、代码注释与文档一律使用中文；标识符、日志与错误码保持英文。
- **提交信息**：首行是一句完整的中文描述，不超过 50 字，可带作用域前缀，例如 `store: 定义会话表与 FTS 索引`、`docs: 补充同步流程说明`；需要说明动机或取舍时写在正文。
- **一次提交只做一件事**：重构与功能分开，格式化改动单独提交。
- **PR**：描述用三到五句话说明做了什么、为什么、哪些部分未验证；CI 必须通过。涉及架构边界（依赖方向、数据所有权、AI 只提议不执行）的改动请先开 Issue 讨论。
- **不要做的事**：不要让 `ui` 直接调用模型；不要在 `assistant` 里出现 IMAP；不要把 AI 产物与来自服务器的事实数据混存；不要提交密钥或 `.env`。

## 代码风格

- Rust：`rustfmt` 默认风格，`clippy` 零警告，公开项写文档注释。
- TypeScript：Vite 默认的严格 TS 配置，`oxlint` 零错误。组件只消费 `@palacemail/ui` 的设计令牌，不写字面量颜色与尺寸。
