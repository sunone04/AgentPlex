# Domain Docs

工程技能在探索代码库时应如何消费本仓库的领域文档。

## 探索前，先读这些

- 仓库根目录的 **`CONTEXT.md`**，或
- 若存在 **`CONTEXT-MAP.md`**（仓库根）—— 它指向每个 context 各自的 `CONTEXT.md`。读取与当前主题相关的每一份。
- **`docs/adr/`** —— 阅读与你即将改动区域相关的 ADR。多 context 仓库还应检查 `src/<context>/docs/adr/` 中的 context 级决策。

若以上文件不存在，**静默继续**。不要标记其缺失，也不要建议预先创建。`/domain-modeling` 技能（经由 `/grill-with-docs` 与 `/improve-codebase-architecture` 触达）会在术语或决策真正落定时懒创建它们。

## 文件结构

Single-context 仓库（绝大多数仓库）：

```
/
├── CONTEXT.md
├── docs/adr/
│   ├── 0001-event-sourced-orders.md
│   └── 0002-postgres-for-write-model.md
└── src/
```

Multi-context 仓库（根目录存在 `CONTEXT-MAP.md`）：

```
/
├── CONTEXT-MAP.md
├── docs/adr/                          ← 全系统级决策
└── src/
    ├── ordering/
    │   ├── CONTEXT.md
    │   └── docs/adr/                  ← context 专属决策
    └── billing/
        ├── CONTEXT.md
        └── docs/adr/
```

## 使用词汇表的术语

当你的输出命名一个领域概念（issue 标题、重构提案、假设、测试名）时，使用 `CONTEXT.md` 中定义的术语，不要漂移到词汇表明确回避的同义词。

若所需概念尚未进入词汇表，这是一个信号 —— 要么你在发明项目未使用的语言（重新考虑），要么存在真实缺口（记录给 `/domain-modeling`）。

## 标记 ADR 冲突

若你的输出与既有 ADR 矛盾，显式指出而非静默覆盖：

> _与 ADR-0007（event-sourced orders）矛盾 —— 但值得重新审视，因为…_
