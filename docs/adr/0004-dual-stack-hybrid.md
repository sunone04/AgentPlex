# 0004: 双栈混合——Rust 执行端 + TypeScript 管理层

产品视角复盘后确认采用**双栈混合架构**：执行层（引擎）用 Rust copy-in Codex（ADR-0001），管理层 Web 控制台用 TypeScript。依据 spec §2.2 双端架构与 §6.2 核心壁垒——执行安全的硬门槛由 Codex 最成熟的沙箱/工具/审计承载，管理层只是其消费视图，用团队更熟悉的前端栈交付。

已弃置仓库既有 12 个 TypeScript issue（#1–12，Electron/NestJS/Docker 单栈方案）：其把执行层放在 Docker 沙箱上，与"恶意命令无法逃逸、桌面端本地执行"的核心卖点直接抵触。原问题保持 open 但不再作为执行基准，Rust 版切片以新 issue 发布。

边界：Rust 负责沙箱、原子工具、执行策略、核心循环、审计入库（SQLite）、headless daemon RPC；TypeScript 负责 Web 控制台（权限治理、看板、监控、审计查询）。二者以本地/远程 RPC + 共享事件流（TurnItem）衔接，不共享代码，仅共享契约。