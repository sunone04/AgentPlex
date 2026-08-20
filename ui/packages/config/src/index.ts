// AgentPlex TS 管理层配置包占位。
// 按 ADR-0004，TS 负责 Web 控制台（权限治理/看板/监控/审计查询），
// 与 Rust 执行端经 RPC + 共享事件流(TurnItem)契约衔接，不共享代码。
// 当前为最小占位实现，保证 workspace 构建与测试全绿。

export interface EngineIdentity {
  name: string;
  /** 架构边界标识，与 CONTEXT.md 词汇表一致。 */
  architecture: 'dual-stack-hybrid';
}

export function engineIdentity(): EngineIdentity {
  return { name: 'agentplex-engine', architecture: 'dual-stack-hybrid' };
}