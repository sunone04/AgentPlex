# ADR-0004: 云端服务采用 NestJS

- **状态**: 已接受（2026-08-19）

## 背景

云端服务承载：Web 控制台 API、RBAC-ABAC 权限（spec §3.1.3）、连接器网关（spec §3.4）、定时任务调度（spec §3.5）、全链路审计（spec §3.7）、监控聚合（spec §3.8）。模块数量多、领域边界清晰。

## 决策

后端框架采用 **NestJS**（Express 底座，TS 原生）。

## 理由

1. **模块化**：连接器、审计、权限、调度天然对应 Nest Module，依赖注入便于隔离测试。
2. **企业级横切能力**：Guard（RBAC）、Interceptor（审计脱敏）、WebSocket Gateway（会话流推送）均为框架内建。
3. **BullMQ 集成**：@nestjs/bullmq 官方集成定时任务调度器。
4. 与 ADR-0001 一致，协议类型直接 import packages/shared。

## 后果

- 正向：模块边界即未来拆分边界（私有化部署时可按模块聚合）。
- 代价：装饰器与 DI 有学习成本；性能上限低于 Fastify 裸用，但管理面流量不构成瓶颈。

## 备选方案

- **Fastify**：轻量高性能，但模块化、Guard、Gateway 需全部自建，企业多模块场景成本更高。
- **Express**：类型支持与结构化能力弱，新项目不采用。
