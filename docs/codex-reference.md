# AgentPlex × Codex 实现参考文档

> **版本**: v1.0
> **日期**: 2026-08-20
> **依据**: [spec.md](./spec.md)（AgentPlex 产品规格 v1.0） + `codex/` 开源仓库（OpenAI Codex CLI, Apache-2.0）
> **目的**: 识别功能重叠模块，提炼成熟设计模式与可直接复用的实现，避免重复造轮子。

---

## 一、分析结论摘要

AgentPlex 与 Codex 在 **执行层（桌面端）** 高度重叠，在 **管理层（Web 控制台 / 企业治理）** 基本不重叠。

| 结论 | 说明 |
|---|---|
| **重度重叠（直接参考）** | 沙箱与权限、Agent 核心循环、原子工具集、Skills 系统、记忆管理、配置与治理、审批流 |
| **中度重叠（借鉴协议与架构）** | 审计/状态存储、双端 RPC 协议、连接器/MCP |
| **基本不重叠（需自研）** | Project 协作看板、协调 Agent、定时任务调度、分级安全响应（L1/L2/L3）、Web 管理控制台 UI、企业 RAG 知识库 |

> **核心策略**：Codex 的开源部分（Rust `codex-rs` 工作区）本身就是"本地执行引擎"的成熟参考实现。AgentPlex 桌面端的执行层应**以 Codex 的 crate 划分与设计模式为蓝本**，而非从零发明。

---

## 二、功能重叠矩阵

| AgentPlex 规格模块（spec 章节） | Codex 对应模块 | 重叠度 | 建议策略 |
|---|---|---|---|
| 3.6 沙箱设计（进程级隔离） | `codex-rs/sandboxing/`、`codex-rs/linux-sandbox/`、`codex-rs/bwrap/` | 🔴 重度 | 直接参考其跨平台抽象 |
| 3.1.3 权限管理（RBAC-ABAC、审批流） | `codex-rs/execpolicy/`、`core/src/tools/approvals.rs`、`core/src/guardian/` | 🔴 重度 | 直接参考决策模型与审批流 |
| 3.1.5 Agent 核心循环 | `codex-rs/core/`（rollout / TurnItem 事件流） | 🔴 重度 | 直接参考事件驱动循环 |
| 3.1.1 原子工具集 | `codex-rs/core/src/tools/`、`codex-rs/tools/` | 🔴 重度 | 直接参考 ToolSpec 机制 |
| 3.1.4 Skills 渐进式加载 | `codex-rs/skills/`、`codex-rs/ext/skills/` | 🔴 重度 | 直接参考 SKILL.md 规范 |
| 3.1.2 记忆管理（三层架构） | `codex-rs/memories/` | 🔴 重度 | 直接参考两阶段管线 |
| 4.1 Agent 配置与人设 | `codex-rs/config/`、`core/src/agent/role.rs` | 🔴 重度 | 直接参考配置分层 + 角色层 |
| 3.7 全链路审计 | `codex-rs/state/`、`codex-rs/otel/` | 🟡 中度 | 借鉴 SQLite + tracing 模式 |
| 2.2 双端架构 | `codex-rs/exec-server/`、`app-server/`、`sdk/typescript/` | 🟡 中度 | 借鉴 RPC 协议约定 |
| 3.4 连接器网关 | `codex-rs/connectors/`、`codex-rs/codex-mcp/`、`mcp-server/` | 🟡 中度 | 复用 MCP 协议为标准 |
| 3.3 Project 协作 / 看板 / 产出审核 | 无（Codex 为单人单 Agent） | ⚪ 无 | 自研 |
| 3.5 定时任务调度 | 无 | ⚪ 无 | 自研 |
| 3.6.3 / 3.7.3 分级响应 L1/L2/L3 | 部分（guardian 仅裁决单次动作） | 🟡 中度 | 借鉴 guardian 模板扩展 |
| 3.8 监控面板 / ROI | 无（仅本地 TUI + 遥测） | ⚪ 无 | 自研 |

---

## 三、重度重叠模块：可直接参考的设计模式与实现

### 3.1 沙箱与权限管理（🔥 直接参考）

**Codex 实现位置**：[sandboxing/src/lib.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/sandboxing/src/lib.rs)、[manager.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/sandboxing/src/manager.rs)、[execpolicy](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/execpolicy/src/lib.rs)

**可借鉴的设计模式**：

1. **平台适配层抽象**（对应 AgentPlex"跨平台沙箱"）：
   - `SandboxType` 枚举：`None / MacosSeatbelt / LinuxSeccomp / WindowsRestrictedToken`（见 [manager.rs#L36-L53](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/sandboxing/src/manager.rs#L36-L53)）。
   - `get_platform_sandbox(windows_sandbox_enabled)` 按 `cfg!(target_os)` 返回平台后端（见 [manager.rs#L62-L76](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/sandboxing/src/manager.rs#L62-L76)）。
   - `SandboxManager::transform()` 把 `SandboxCommand`（program/args/cwd/env）转换成 `SandboxExecRequest`（含 sandbox 类型 + 权限配置），**在 exec-server 执行边界才落地为原生进程**。

2. **权限即数据结构 `PermissionProfile`**：
   - 一条权限配置 = 文件系统策略（可读根/可写根）+ 网络策略 + enforcement（强制等级），一次传给沙箱层。
   - 用 `SandboxablePreference::Auto/Require/Forbid` 表达"是否允许在沙箱内运行"。

3. **声明式执行策略语言（Starlark）**：
   - 规则写在 `~/.codex/rules/*.rules`，语法为 `prefix_rule(pattern=[...], decision="allow|prompt|forbidden", match=[...], not_match=[...])` 与 `network_rule(...)`。
   - 核心模型：`PrefixPattern`（首 token 固定 + 后续 `PatternToken::Single/Alts` 前缀匹配）、`Rule` trait（`program()` + `matches()`）、`Decision` 三态（见 [decision.rs#L7-L16](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/execpolicy/src/decision.rs#L7-L16)）。
   - **多条规则命中取最严格决策**：`forbidden > prompt > allow`（这正是 AgentPlex"企业级黑名单永远生效"的可落地实现）。
   - 规则自带 `match`/`not_match` 内联单元测试，加载时自动校验，防止规则写错。

4. **环境注入而非信任子进程**：`exec_env.rs` 的 `create_env()` 依据 `ShellEnvironmentPolicy` 构造白名单环境；`CODEX_PERMISSION_PROFILE` 环境变量仅为信息用途（"must not be treated as proof of enforcement"）——**沙箱靠 OS 边界，不靠环境变量**。

5. **审批流类型化**：`ApprovalAction` 枚举把每种待审批动作建模为强类型（Shell / ExecCommand / Execve / ApplyPatch / McpToolCall / NetworkAccess / RequestPermissions），审批上下文 `ApprovalContext` 携带工具名、理由、重试原因（见 [approvals.rs#L66-L150](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/core/src/tools/approvals.rs#L66-L150)）。已批准命令写入 `default.rules` 持久化（`blocking_append_allow_prefix_rule`）。

6. **LLM-as-a-Judge（Guardian）**：`core/src/guardian/` 的裁决提示模板定义了三要素：`risk_level`（low/medium/high/critical）+ `user_authorization`（high/medium/low/unknown）+ 租户安全策略，最后按阈值推导 `allow/deny`（见 [policy_template.md](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/core/src/guardian/policy_template.md)）。**这直接对应 AgentPlex 规格"规划层：策略引擎校验（LLM-as-a-Judge）"**。

> **AgentPlex 落地建议**：直接照搬 `SandboxType` + `SandboxManager::transform` + `Decision` 三态 + 声明式规则 + Guardian 裁决模板。企业级"Deny 永远生效"通过"规则分层（企业层规则不可被低层覆盖）+ 最严格决策合并"实现。

---

### 3.2 Agent 核心循环（🔥 直接参考）

**Codex 实现位置**：[core/src/rollout.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/core/src/rollout.rs)、[protocol/src/items.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/protocol/src/items.rs)、[core/src/exec.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/core/src/exec.rs)

**可借鉴的设计模式**：

1. **事件流驱动（TurnItem 类型化事件）**：一次 Agent 交互被建模为有序的类型化事件流，`TurnItem` 是有 tag 的联合类型（见 [items.rs#L44-L75](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/protocol/src/items.rs#L44-L75)）：
   - `UserMessage / AgentMessage / Reasoning / Plan / CommandExecution / DynamicToolCall / McpToolCall / FileChange / ContextCompaction / WebSearch / ImageView / Extension ...`
   - **一条事件 = 一条审计记录 = 一条可渲染 UI 数据 = 一条可持久化记录**。这正是 AgentPlex"全链路审计（谁+何时+工具+目标+结果）"的天然载体，避免为审计单独建一套系统。

2. **执行输出防爆（Token 治理）**：
   - `ExecCapturePolicy::ShellTool/FullBuffer` 区分"给模型看的输出"与"内部工具输出"（见 [exec.rs#L110-L118](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/core/src/exec.rs#L110-L118)）。
   - 默认超时 `DEFAULT_EXEC_COMMAND_TIMEOUT_MS = 10_000`、输出硬上限 `EXEC_OUTPUT_MAX_BYTES`、`IO_DRAIN_TIMEOUT_MS = 2_000` 防止孙进程挂起。
   - `format_exec_output_for_model()` 把输出格式化为 `Exit code / Wall time / Output` 并截断——返回给模型的执行结果有统一元数据。

3. **上下文预算纪律**（AGENTS.md 硬规则）：上下文只增不减（No history rewrite）、每项有界、单项不超过 10K token、所有注入片段必须是实现 `ContextualUserFragment` trait 的结构体。**这是企业级长会话稳定性的关键工程纪律**。

4. **执行请求统一结构**：`ExecRequest / ExecParams` 把命令、cwd、过期策略（Timeout/DefaultTimeout/Cancellation）、捕获策略、沙箱权限、网络代理打包成一个对象传递，避免参数层层穿透。

> **AgentPlex 落地建议**：以 `TurnItem` 联合类型作为桌面端与审计、UI、存储的唯一中间格式；以 rollout（一次会话执行记录）为审计粒度单元。

---

### 3.3 原子工具集（🔥 直接参考）

**Codex 实现位置**：[tools/src/tool_spec.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/tools/src/tool_spec.rs)、[tools/src/lib.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/tools/src/lib.rs)、[core/src/tools/mod.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/core/src/tools/mod.rs)

**可借鉴的设计模式**：

1. **统一工具规范 `ToolSpec`**：枚举 `Function / Namespace / ToolSearch / WebSearch / Freeform`，序列化后直接是 OpenAI Responses API 的合法 Tool（见 [tool_spec.rs#L21-L56](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/tools/src/tool_spec.rs#L21-L56)）。工具参数用 `JsonSchema` 描述。

2. **工具命名空间 `ToolName`**：`namespace + name` 双层命名，默认命名空间 + 扩展命名空间，支撑海量工具不冲突（见 `flat_tool_name()`）。

3. **工具执行器抽象**：`ToolDefinition / ToolCall / ToolExecutor / ToolOutput` 分层——工具**声明**（schema）、**调用**（参数解析）、**执行**（具体实现）解耦，MCP 工具只是普通工具的封装（`mcp_tool.rs` 把外部工具转成内部 ToolSpec）。

4. **Code Mode（沙箱内工具调用）**：`tools/src/code_mode.rs` 用 V8 在沙箱内执行工具调用（`code_mode_name_for_tool_name` 等），工具逻辑运行在受控 JS 运行时中——比"Agent 直接调宿主 API"更安全。

5. **文件写入首选 apply_patch**：`apply_patch` 工具以 git diff 形式表达文件变更，天然可审批、可审计、可展示 diff。

> **AgentPlex 落地建议**：工具集直接复用"ToolSpec 声明 + ToolName 命名空间 + ToolExecutor 抽象"三层结构。Codex 内置工具（shell / apply_patch / web_search / web_fetch / mcp 工具 / skills）即 AgentPlex 原子工具集的 80% 内容，**不要重写**，只在其上增加企业级包装（审批、审计、策略）。

---

### 3.4 Skills 渐进式加载（🔥 直接参考）

**Codex 实现位置**：[skills/src/loading.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/skills/src/loading.rs)、[skills/src/parser.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/skills/src/parser.rs)、[SKILL.md 示例](file:///g:/Agent/OpenWay-Agent/codex/.codex/skills/test-tui/SKILL.md)

**可借鉴的设计模式**：

1. **SKILL.md 规范（直接复用）**：YAML frontmatter（`name` / `description` / `metadata.short-description`）+ 正文指令。解析器 `parse_skill_frontmatter_metadata()` 校验 name ≤ 64 字符、description 必填、自动修复第三方技能的容错 YAML（见 [parser.rs#L44-L92](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/skills/src/parser.rs#L44-L92)）。**这正是 AgentPlex"Skills 三级仓库"的文件级标准，可直接照搬**。

2. **目录即仓库**：`.codex/skills/<skill-name>/SKILL.md` 约定（项目级），另有用户级 `~/.codex/skills`。多根加载按优先级合并（`SkillRootLoader::load_roots`，见 [loading.rs#L113-L115](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/skills/src/loading.rs#L113-L115)）。

3. **渐进式两层加载**：
   - **索引层**：`LoadedSkillRoot` 只含 name + description + path，预置注入上下文（对应 AgentPlex"索引层每次对话预置"）。
   - **内容层**：SKILL.md 完整指令**按需**加载，且带缓存快照（`SkillRootSnapshots` 用 Arc 指针身份做缓存 key）。
   - 加载不因单个坏技能失败：`LoadedSkills { skills, errors }` 收集错误、继续加载其余技能。

4. **技能提及注入（mentions）**：从文本中识别 `@skill` 提及并注入相关技能上下文（`skills/src/mentions.rs`）。

> **AgentPlex 落地建议**：SKILL.md 格式、目录发现、两层加载、错误容忍、缓存快照全部直接复用。三级仓库（官方/企业/个人）只需在"多根加载优先级"上扩展一层企业根目录，并叠加 IT 审核（上传时安全扫描）。

---

### 3.5 记忆管理（🔥 直接参考）

**Codex 实现位置**：[memories/README.md](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/memories/README.md)、`core/src/memories/`

**可借鉴的设计模式**（两阶段异步管线）：

1. **Phase 1 — 逐会话抽取（可扩展）**：从状态库选择符合条件的历史会话，并行送模型生成**结构化记忆**（`raw_memory` 明细 + `rollout_summary` 摘要 + 可选 `slug`）。用 **DB job 租约（claim/lease）** 防止并发重复处理，失败带退避重试。

2. **Phase 2 — 全局合并（串行安全）**：全局锁 + git 基线工作区（`~/.codex/memories/.git`），生成 `raw_memories.md` + `rollout_summaries/`，用 git diff（`phase2_workspace_diff.md`）驱动一个**专门的合并子 Agent**（无审批、无网络、仅本地写）做最终合并。空变更则跳过。

3. **关键设计决策**：记忆不是对话时即时写入，而是**事后异步抽取**——不阻塞主循环、可分批、可重试、由模型产出结构化记忆。分层记忆（会话/项目/企业）可在 Phase 2 的"记忆工作区"上扩展不同命名空间。

> **AgentPlex 落地建议**：照搬"异步两阶段 + DB 租约 + git 基线 diff"管线。项目级共享记忆 = 把记忆工作区放到 Project 云端共享路径；企业级 RAG = 在 Phase 2 之上接向量检索。

---

### 3.6 配置与治理（🔥 直接参考）

**Codex 实现位置**：[config/src/config_layer_source.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/config/src/config_layer_source.rs)、[core/src/agent/role.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/core/src/agent/role.rs)、`config/src/merge.rs`

**可借鉴的设计模式**：

1. **配置分层栈（对应 AgentPlex 企业级/Project 级/Agent 级三层权限）**：`ConfigLayerSource` 枚举（见 [config_layer_source.rs#L6-L24](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/config/src/config_layer_source.rs#L6-L24)）：
   ```
   PackagedDefaults → Mdm(设备管理) → System → EnterpriseManaged(企业云包) → User → Project(.codex) → SessionFlags
   ```
   低层不可覆盖高层。`requirements.toml` 承载企业强制约束（如 `allow_managed_hooks_only`）。**这正是 AgentPlex"企业级 > Project 级 > Agent 级"权限冲突规则的配置化实现**。

2. **Agent 角色 = 配置层叠加**：`AgentRoleConfig`（name/description/config_file），内置角色 `default / explorer / worker` + 用户自定义角色（见 [role.rs#L380-L445](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/core/src/agent/role.rs#L380-L445)）。角色在 spawn 时以高优先级配置层注入，可锁定 model/reasoning effort/service tier。**这直接对应 AgentPlex"人设设定 + 企业模板锁定"**。

3. **配置合并不手写**：`ConfigToml` 分层 merge + `just write-config-schema` 自动生成 JSON Schema，配置变更受 schema 约束。

> **AgentPlex 落地建议**：企业模板库 = `EnterpriseManaged` 配置层 + 角色（人设）文件；Project 限制 = `Project` 层；员工微调 = `User` 层 + SessionFlags。锁定的核心行为准则（代码规范等）放进企业层不可覆盖字段。

---

## 四、中度重叠模块：借鉴协议与架构

### 4.1 审计与状态存储

**Codex 实现位置**：[state/src/model/log.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/state/src/model/log.rs)、[state/src/audit.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/state/src/audit.rs)、[state/src/sqlite.rs](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/state/src/sqlite.rs)

- 用 **SQLite 统一存会话/线程/日志**（`threads` 表 + `LogRow` 结构化日志查询 `LogQuery`）。
- `RolloutRecorder` 记录一次 Agent 会话的完整事件，天然满足"全链路审计"粒度。
- `otel` crate 提供 tracing + OTEL exporter，为监控面板（Token 消耗、使用记录）打底。
- **借鉴点**：审计不必另起炉灶——TurnItem 事件流 + SQLite 即可得到完整审计；脱敏可在写入层统一做（AgentPlex 规格的"密码/Token 脱敏"在此实现）。

### 4.2 双端架构与 RPC 协议

**Codex 实现位置**：[exec-server](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/exec-server/src/lib.rs)、[app-server](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/app-server/src/lib.rs)、[sdk/typescript](file:///g:/Agent/OpenWay-Agent/codex/sdk/typescript/src/index.ts)

- **本地执行与远端控制分离**：`exec-server`（本地沙箱执行）+ `app-server`（管理/API 面），RPC 方法约定 `<resource>/<method>`、camelCase、时间戳 `*_at`、游标分页（`cursor/limit/next_cursor`）。
- **TS SDK 自动生成**：Rust 类型标注 `#[ts(export_to=...)]` 直接生成 TypeScript 类型，前后端契约单一来源。
- **借鉴点**：AgentPlex 桌面端 ↔ Web 控制台的双端架构可直接套用 `exec-server + app-server + 自动生成 SDK` 的分层；桌面端即 exec-server 的超集（加 Workspace 绑定）。

### 4.3 连接器 / MCP

**Codex 实现位置**：[connectors](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/connectors/src/lib.rs)、[codex-mcp](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/codex-mcp/src/lib.rs)、[mcp-server](file:///g:/Agent/OpenWay-Agent/codex/codex-rs/mcp-server/src/lib.rs)

- **直接复用 MCP 协议作为连接器标准**：`parse_mcp_tool()` 把外部 MCP 工具包装为内部 ToolSpec，工具审批携带连接器身份（`connector_id/connected_account_email`）。
- **借鉴点**：AgentPlex"企业连接器网关" = MCP 服务 + 平台侧凭证管理。Codex 已解决"工具协商/工具发现/审批携带身份"；AgentPlex 只需在 MCP 服务侧加凭证库（IT 管理员统一配置 OAuth/Token）。

---

## 五、基本不重叠（需 AgentPlex 自研）

| 模块 | 说明 | 可参考的外部方向 |
|---|---|---|
| Project 协作空间/看板/产出审核 | Codex 是单人单 Agent | Trello/Linear 交互 |
| 协调 Agent 结构化消息通信 | Codex 的 subagent 是自然语言返回 | 可借鉴 TurnItem 类型化扩展为 Agent↔Agent 消息 |
| 定时任务（云端调度 + 本地执行混合） | 无对应 | 参考 WorkBuddy |
| 分级安全响应 L1/L2/L3 | guardian 只裁决单次动作 | 在 guardian/审计之上加"策略引擎" |
| Web 管理控制台（监控/ROI/权限治理） | 无 | 自研（数据源 = state SQLite + 遥测） |
| 企业 RAG 知识库 | 无 | 在记忆 Phase 2 之上接向量库 |

---

## 六、直接复用清单与落地路径

### 6.1 建议直接复用的 Codex 设计资产

| 资产 | 位置 | 复用方式 |
|---|---|---|
| 沙箱抽象 | `sandboxing/`（SandboxType/SandboxManager/PermissionProfile） | 代码级复用或移植 |
| 执行策略引擎 | `execpolicy/`（Decision/PrefixRule/Starlark 规则） | 代码级复用 |
| Guardian 裁决模板 | `core/src/guardian/policy_template.md` | 直接作为 LLM-as-Judge 提示词基座 |
| SKILL.md 规范与加载器 | `skills/`（parser/loading/mentions） | 规范与代码级复用 |
| 记忆管线设计 | `memories/README.md` | 架构级复用 |
| 配置分层栈 | `config/`（ConfigLayerSource + merge） | 架构级复用 |
| 事件流模型 | `protocol/src/items.rs`（TurnItem） | 规范级复用 |
| 执行输出治理 | `core/src/exec.rs`（timeout/cap/format） | 常量与模式级复用 |
| 双端 RPC 约定 | `app-server/` v2 规范 | 规范级复用 |

### 6.2 建议的 Phase 1 落地路径（对应 spec 第七章节）

1. **优先移植**：`sandboxing` + `execpolicy` + `core/src/exec.rs`（输出治理）——这是"桌面端执行引擎"的骨架。
2. **再移植**：`skills`（SKILL.md 加载）+ `tools`（ToolSpec/ToolName/ToolExecutor）+ `state`（SQLite 审计）。
3. **随后接入**：`guardian`（LLM-as-Judge）作为审批中枢；`memories` 两阶段管线作为记忆层。
4. **最后自研**：Project 协作、定时任务、Web 控制台——数据全部建立在第 2 步的 SQLite + TurnItem 事件流之上。

### 6.3 避坑提醒

- **不要重写原子工具**：Codex 的 shell/apply_patch/web_search 已成熟，直接在此基础上加企业包装。
- **上下文预算纪律**：沿用"单注入项 ≤ 10K token、总注入有界、上下文只增不减"的硬规则，避免企业长会话退化。
- **不要把审批信任放环境变量**：沙箱强制靠 OS 边界（bwrap/seatbelt/受限 Token），环境变量仅作信息传递。
- **保持事件流单一来源**：审计/UI/存储共用 TurnItem，避免三套数据模型。

---

*本文档基于 `codex/` 仓库当前代码分析生成，作为 AgentPlex 开发时避免重复造轮子的实现基准。*
