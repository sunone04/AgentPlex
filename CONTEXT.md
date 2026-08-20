# AgentPlex

企业级 AI Agent 平台（AI OS）：自下而上地执行 Agent 任务、自上而下地治理其权限与生命周期。本文档是本项目领域词汇的唯一来源。

## Language

**Agent（执行面）**:
一个配置好的 AI 角色资源，含人设、Skills、连接器与授权源引用。执行层的一个受治理的工作主体。
_Avoid_: bot, worker 实例

**Engine（执行引擎）**:
桌面端本地负责落地 Agent 动作的进程，运行于沙箱边界内，通过本地 RPC 被消费。AgentPlex 的执行端契入是对成熟 Codex 引擎的选择性继承。
_Avoid_: runtime（避免与模型运行时混淆）

**Decision（强制原语）**:
对单个动作在运行时的强制判定，三值：`allow / prompt / forbidden`。这是执行层唯一可强制实施的原语，任何授权规则最终都折算成它。
_Avoid_: 权限判定、判定结果（避免与授权来源混淆）

**Authorization Source（授权来源）**:
决定某个动作“在此上下文是否被允许”的*输入*，按主体分层：Agent 级 / Project 级 / 企业级。与 Decision 的关系：授权来源算 authorization，Decision 做 enforcement。
_Avoid_: 权限（overloaded）

**授权分层（Authorization Layers）**:
授权来源按作用域分层的语义：Agent 级（员工自有）、Project 级（Owner 治理）、企业级（IT/安全先生成，Deny 永远生效）。分层是输入侧的概念，不等于决策原语。
_Avoid_: 权限层级

**Workspace（工作空间）**:
Agent 任务执行所处的作用域。个人模式绑定本地文件夹；Project 模式为云端共享目录。Engine 的沙箱边界在此之上界定。
_Avoid_: folder（过于具体）、space

**Skill**:
一个专项能力包，规范为 `SKILL.md`（frontmatter + 指令正文），采用渐进式懒加载。可直接沿用成熟 Codex 的 Skills 规范。
_Avoid_: 技能插件、plugin

**Connector（连接器）**:
对接企业内部系统的受信服务，凭证由 IT 管理员统一配置，供 Agent 在授权范围内启用。以 MCP 协议为互操作标准。
_Avoid_: integration（避免与外部产品混淆）、插件

**Memory（记忆）**:
跨会话持久保留的上下文，按层级组织（会话 / 项目 / 企业 RAG），经由异步抽取生成而非对话时即时写入。
_Avoid_: context（仅指单一窗口）

**Rollout**:
一次 Agent 会话（用户触发到完成）的完整生命周期单元，是审计、价值统计与事件流的基础粒度。
_Avoid_: session（承载 JS web 语义、与此不同）

**Model Gateway（模型网关）**:
Engine 与任意供应商模型之间的抽象层。以 trait 接口解耦具体模型；首发 OpenAI 与 DeepSeek（同兼容协议），其余模型以适配器接入。
_Avoid_: LLM provider（仅商业概念，非架构边界）

**Multi-tier Response（分级响应）**:
异常行为的 L1/L2/L3 自动响应（告警员工 / 暂停 Agent 通知主管 / 隔离冻结通知安全团队）。Phase 2 治理能力，执行层不直接实现。
_Avoid_: alert escalation（到期阶段再命名）