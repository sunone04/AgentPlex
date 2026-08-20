import { describe, expect, it } from 'vitest';
import { engineIdentity } from '../src/index.js';

describe('engineIdentity', () => {
  it('为 dual-stack-hybrid 架构返回执行端标识', () => {
    const id = engineIdentity();
    expect(id.name).toBe('agentplex-engine');
    expect(id.architecture).toBe('dual-stack-hybrid');
  });
});