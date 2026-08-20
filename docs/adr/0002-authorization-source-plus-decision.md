# 0002: 授权 = 分层授权源 + Decision 强制原语

spec 的 RBAC-ABAC 三级权限模型与 Codex 的逐动作 `Decision`（allow/prompt/forbidden）不是同一层面的东西，本 ADR 明确二者关系：**分层授权源（authorization，输入）决定“该动作在此上下文是否被允许”；Decision（enforcement，运行时原语）决定“被放行的动作在运行时如何落地强制”。**

改造后的统一模型：`Decision` 是执行层唯一可强制实施的原语；RBAC-ABAC 的 Agent/Project/企业三级身份与规则折算为 Decision 的输入——企业黑名单 → `forbidden`，Agent 预授权 → `allow`，未决 → `prompt`。分层沿 codex `ConfigLayerSource` 配置层栈落位，低层不可覆盖高层以承载“企业 Deny 永远生效”。

Phase 1 只实现 Agent 级授权源；Project/企业层以空占位层存在，接口先行、实现后续，不改变决策原语模型。