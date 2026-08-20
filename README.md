# AgentPlex — 企业级 AI Agent 平台

企业级 AI OS：管理 AI Agent 的权限与生命周期，进入人机协同时代。

本仓库为**双栈混合 Monorepo**（[ADR-0004](docs/adr/0004-dual-stack-hybrid.md)）：

- `crates/` — **Rust 执行端**：桌面执行引擎（沙箱、原子工具、执行策略、核心循环、审计、headless daemon）。选择性 copy-in 成熟 Codex 引擎（[ADR-0001](docs/adr/0001-reuse-codex-engine.md)）。
- `ui/` — **TypeScript 管理层**：Web 控制台（权限治理、看板、监控、审计查询）。

文档入口：[产品规格](docs/spec.md) · [领域词汇](CONTEXT.md) · [架构决策](docs/adr/)

## 开发环境要求

- [Rust](https://rustup.rs) (1.75+)
- [Node.js](https://nodejs.org) ≥ 20
- [pnpm](https://pnpm.io) 9+

## 一条命令构建全绿

```bash
# Rust 执行端（cargo build + test）
cargo build --workspace --all-targets && cargo test --workspace

# TS 管理端（测试 + 类型检查；需先 install）
cd ui
pnpm install --store-dir .pnpm-store-local
pnpm -r test && pnpm -r lint
cd ..
```

> 注：Rust 与 TS 是两个独立 workspace，各跑各的构建/测试，互不阻塞。`ci` 中的完整检查（含 clippy -D warnings 与 rustfmt --check）请以每次在根目录执行 `cargo clippy --workspace --all-targets -- -D warnings` 与 `cargo fmt --all -- --check` 复现。

## CI

GitHub Actions（`.github/workflows/ci.yml`）在 main 分支 push 与 PR 时自动运行，包含 Rust（build/test/clippy/fmt）与 TypeScript（install/test/typecheck）两组检查。