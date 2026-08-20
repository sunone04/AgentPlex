# 0003: Phase 1 = 模型网关 + headless daemon + Windows 优先

Phase 1 在抽象模型网关的边界（trait 接口解耦具体模型，首发 OpenAI 与 DeepSeek，二者同 OpenAI 兼容协议，其余以适配器接入），而 Origin 的 `core` 深度绑定 OpenAI Responses API——因此 copy-in 时需把模型客户端一层的借用替换为网关 trait。投放形态为 **headless daemon + 本地 RPC**（沿用 codex `exec-server`/`app-server` 骨架），直接契入 AgentPlex 双端架构，Web 控制台与未来桌面壳只是该 daemon 的客户端。优先验收 **Windows**（受限 Token 沙箱），同时保留跨平台沙箱抽象（`SandboxType` 三后端均 copy-in）。

关键取舍与风险：Windows 沙箱的进程隔离能力弱于 macOS seatbelt / Linux bwrap，直接冲击“即便恶意命令也无法逃逸”的核心承诺。接受该风险并将「Windows 弱沙箱兜底策略」作为显式待办（Phase 1 落地前必须给出受控降级方案）；Guardian（LLM-as-Judge 规划层裁决）推迟到 Phase 2，Phase 1 依赖“沙箱 + Decision enforcement”硬隔离兜底。