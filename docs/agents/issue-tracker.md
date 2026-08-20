# Issue tracker: GitHub

本仓库的 issue 与 spec 以 GitHub issues 承载。所有操作使用 `gh` CLI。

## 约定

- **创建 issue**：`gh issue create --title "..." --body "..."`。多行正文使用 heredoc。
- **读取 issue**：`gh issue view <number> --comments`，用 `jq` 过滤评论并同时获取 labels。
- **列出 issues**：`gh issue list --state open --json number,title,body,labels,comments --jq '[.[] | {number, title, body, labels: [.labels[].name], comments: [.comments[].body]}]'`，配合 `--label` 与 `--state` 过滤。
- **评论 issue**：`gh issue comment <number> --body "..."`
- **增删标签**：`gh issue edit <number> --add-label "..."` / `--remove-label "..."`
- **关闭**：`gh issue close <number> --comment "..."`

仓库从 `git remote -v` 推断 —— 在克隆内运行时 `gh` 自动识别。

## Pull requests as a triage surface

**PRs as a request surface: no.**（若本仓库将外部 PR 视为功能请求，可改为 `yes`；`/triage` 会读取此标志。）

## 当技能说 "publish to the issue tracker"

创建一条 GitHub issue。

## 当技能说 "fetch the relevant ticket"

运行 `gh issue view <number> --comments`。
