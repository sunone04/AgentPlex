# ADR-0002: pnpm workspace + Turborepo 的 Monorepo 结构

- **状态**: 已接受（2026-08-19）

## 背景

三端应用（desktop/server/console）+ 多个共享包（core/sandbox/tools/ui/shared）需要在同一仓库内统一版本、共享协议类型、增量构建。仓库布局一旦沉淀大量代码后迁移成本极高。

## 决策

采用 **pnpm workspace + Turborepo**，结构如下：

```
agentplex/
├── apps/
│   ├── desktop/   # Electron 桌面端
│   ├── server/    # NestJS 云端服务
│   └── console/   # React Web 管理控制台
├── packages/
│   ├── core/      # Agent 核心循环、工具协议（双端共享）
│   ├── sandbox/   # 沙箱抽象层（Docker / 受限进程双实现）
│   ├── tools/     # 原子工具集
│   ├── ui/        # 共享组件库
│   └── shared/    # 类型、协议、工具函数
├── docker/        # Dockerfile + compose + helm
└── docs/
```

## 理由

1. 与参考项目 codex 的 pnpm workspace 工具链一致（实现.md 约定）。
2. Turborepo 提供构建缓存与依赖图调度，CI 随包数增长仍可控。
3. packages/core 与 packages/sandbox 独立于 apps，是"云端沙箱降级执行"复用的前提。

## 后果

- 正向：协议类型（工具调用、审计事件）单一定义，双端不会漂移。
- 代价：贡献者需熟悉 pnpm；Node 版本与 pnpm 版本需以 packageManager 字段锁死。

## 备选方案

- **Nx**：生成器与约束更强，但概念负担重，本项目包规模用不上。
- **多仓库**：跨仓库协议同步成本在企业级审计/权限场景不可接受。
