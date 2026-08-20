//! AgentPlex 桌面执行引擎 (Rust) 入口占位 crate。
//!
//! 按 ADR-0001/0004，执行层将选择性 copy-in 成熟 Codex 的重叠 crate
//! (sandboxing / execpolicy / tools / skills / config / state / protocol /
//! exec-server / mcp-server) 作为引擎地基。T2 落地前，本 crate 只提供
//! 最小占位实现，确保 workspace 构建与测试全绿。

/// 引擎版本锚点，供构建信息与诊断使用。
pub fn engine_name() -> &'static str {
    "agentplex-engine"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_name_is_agentplex() {
        assert_eq!(engine_name(), "agentplex-engine");
    }
}
