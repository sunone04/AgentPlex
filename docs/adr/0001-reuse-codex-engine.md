# 0001: 选择性继承 Codex 引擎

AgentPlex 的桌面执行引擎不重造轮子，而是对成熟、开源的 Codex 项目（Apache-2.0）做**选择性 copy-in**：只把高度重叠的 crate（`sandboxing` / `execpolicy` / `tools` / `skills` / `config` / `state` / `protocol` / `exec-server` / `mcp-server` 等）以其底层移植为 AgentPlex 自有 workspace 的地基，保留 Apache-2.0 署名；不 inherit 整个 codex 仓库（含无关的 TUI 与 stub crate）。

不选 `git` 依赖，是因为令人信服的企业级改在（L1/L2/L3 分级响应、企业黑名单“永远生效”、连接器网关）都需要进入这些 crate 内部实现，依赖 vector 无法改造；不选纯模式借鉴，是因为那违反“避免重复造轮子”。

Consequences: 与上游 drift 需自行管理；被 copy-in 的 crate 成为 AgentPlex 维护责任。后续新增重量级能力优先复用 codex 生态而非自研。