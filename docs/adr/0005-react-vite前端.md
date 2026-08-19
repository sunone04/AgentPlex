# ADR-0005: 前端采用 React 18 + Vite

- **状态**: 已接受（2026-08-19）

## 背景

前端覆盖两处：Web 管理控制台（apps/console）与 Electron 渲染层（apps/desktop 的会话 UI）。需共享组件库（packages/ui）与可视化（监控仪表盘）生态。

## 决策

前端统一 **React 18 + Vite**，组件库 Ant Design 5（对齐用户界面偏好：白色主色调、简洁标准风格），状态管理 Zustand，数据请求 TanStack Query。

## 理由

1. **双端共享**：console 与 desktop 渲染层共用 packages/ui，React 生态（虚拟列表、图表、Markdown/diff 渲染）对"会话流 + 审计表格 + 仪表盘"三类界面支撑最全。
2. **Vite** 与 Electron（vite-plugin-electron）、Turborepo 无缝集成，HMR 体验一致。
3. AntD 5 的 Table/Form/ProComponents 可快速搭出管理控制台的权限配置、审计查询界面。

## 后果

- 正向：一套心智模型覆盖两处 UI；diff/Markdown 渲染有成熟方案（会话回放需要）。
- 代价：React 生态迭代快，需锁定版本并定期升级。

## 备选方案

- **Vue 3 + Element Plus**：同样能胜任，团队 React 经验占优时选 React。
