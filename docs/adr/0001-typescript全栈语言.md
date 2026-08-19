# ADR-0001: 采用 TypeScript 作为全栈语言

- **状态**: 已接受（2026-08-19）
- **决策人**: 产品/架构共同确认

## 背景

平台由三端构成：Electron 桌面端执行引擎、NestJS 云端服务、React Web 管理控制台，另有跨端共享的 Agent 核心循环、工具协议、沙箱抽象。语言选型决定了能否让"一次定义、三端复用"成立，属于最早锁定的不可逆决策。

## 决策

全栈统一使用 **TypeScript**（strict 模式），包括桌面端、云端服务、Web 前端、共享 packages 与连接器 SDK。

## 理由

1. **跨端复用**：packages/core（Agent 循环、工具协议）在桌面端与云端沙箱降级执行（spec §3.5）中必须同构运行，TS 是唯一无需跨语言移植的选项。
2. **生态**：Vercel AI SDK、LangChain.js、MCP TypeScript SDK 等 Agent 生态核心库均为 TS 优先。
3. **团队与参考项目**：codex 仓库内含 pnpm workspace 的 TypeScript SDK（codex/sdk/typescript），工具链可直接对齐。
4. **SDK 交付**：企业自研连接器 SDK（spec §3.4）用 TS 发布 npm 包，交付成本最低。

## 后果

- 正向：类型协议单一定义；招聘面宽；Monorepo 内无 FFI 边界。
- 风险：CPU 密集型沙箱隔离、大规模日志处理性能弱于 Rust——接受该代价，必要时局部以 Worker/Rust WASM 补齐，但不改变主语言。

## 备选方案

- **Rust**（codex-rs 同路线）：性能与安全最优，但开发效率、生态、三端复用成本不匹配 MVP 节奏，仅作为局部性能补丁选项。
- **Go**：后端优秀，但桌面端与前端共享逻辑无法复用。
