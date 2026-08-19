# ADR-0003: 桌面端执行引擎采用 Electron

- **状态**: 已接受（2026-08-19）

## 背景

spec §2.2 要求桌面端承担执行引擎职责：Workspace 绑定、沙箱运行、文件操作、本地工具调用，且会话路由云端 LLM。候选：Electron vs Tauri 2。

## 决策

桌面端采用 **Electron**，Node 主进程直接承载执行引擎（工具层、沙箱调度、会话状态），渲染层用 React。

## 理由

1. **语言一致性**：执行引擎核心（packages/core、packages/tools、packages/sandbox）为 TS，Electron 主进程可原生复用；Tauri 需将执行引擎改写为 Rust sidecar，与 ADR-0001 冲突。
2. **生态对齐**：Cursor、VS Code 均为该路线，Node 侧 child_process / dockerode / 文件系统能力成熟。
3. **企业桌面场景**：内存与体积劣势在企业办公机上可接受；换取完全一致的 TS 工具链。

## 后果

- 正向：执行引擎代码零移植；npm 生态全量可用；升级 Chromium/Node 同步。
- 风险：安装包 ~100MB、内存占用偏高；需在主进程中严格区分特权代码与渲染层（contextIsolation 开启，工具调用走 IPC 桥）。

## 备选方案

- **Tauri 2**：体积小、内存低、安全模型好，但核心逻辑须 Rust，跨语言成本贯穿整个产品生命周期，放弃。
