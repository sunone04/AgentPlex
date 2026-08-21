# Third-Party Notices

本文件记录 AgentPlex 仓库中来自第三方开源项目的代码拷贝（copy-in），及其来源与修改情况。
依据 ADR-0001，AgentPlex 桌面执行引擎以**选择性 copy-in** 方式复用成熟开源项目，
并保留原始许可署名。

---

## Codex (OpenAI) — Apache-2.0

- **上游仓库**: https://github.com/openai/codex
- **来源 commit**: `bc7a4870398ac8f7ac90aaab1dc10ee0766f7fe1`
  （2026-08-18, "Centralize persisted resume settings lookup (#39147)"）
- **拷贝时间**: 2026-08-20（T2, issue #15）
- **许可证**: Apache-2.0（各 crate 的 `Cargo.toml` 均声明 `license.workspace = true`，
  workspace 级为 `Apache-2.0`）
- **上游许可文本**: https://github.com/openai/codex/blob/main/LICENSE

### 拷贝范围

以下 crate 从上游 `codex-rs/` 工作区拷贝至本仓库 `crates/`，作为 AgentPlex 执行引擎地基
（保留上游 crate 命名 `codex-*` 以便后续 diff 上游）：

| AgentPlex 路径 | 上游路径 | 说明 |
|---|---|---|
| `crates/sandboxing` | `codex-rs/sandboxing` | 跨平台沙箱抽象（T2 目标 crate） |
| `crates/execpolicy` | `codex-rs/execpolicy` | Starlark 执行策略引擎（T2 目标 crate） |
| `crates/tools` | `codex-rs/tools` | ToolSpec/ToolName/ToolExecutor（T2 目标 crate） |
| `crates/skills` | `codex-rs/skills` | SKILL.md 解析与渐进加载（T2 目标 crate） |
| `crates/config` | `codex-rs/config` | 配置分层栈（T2 目标 crate） |
| `crates/state` | `codex-rs/state` | SQLite 状态/审计存储（T2 目标 crate） |
| `crates/protocol` | `codex-rs/protocol` | TurnItem 事件流协议（T2 目标 crate） |
| `crates/exec-server` | `codex-rs/exec-server` | 本地执行 server（T2 目标 crate） |
| `crates/agent-identity` | `codex-rs/agent-identity` | Agent 身份密钥 |
| `crates/codex-api` | `codex-rs/codex-api` | API 客户端封装 |
| `crates/codex-client` | `codex-rs/codex-client` | 客户端桩 |
| `crates/async-utils` | `codex-rs/async-utils` | 异步工具 |
| `crates/code-mode` | `codex-rs/code-mode` | Code Mode 工具调用 |
| `crates/code-mode-protocol` | `codex-rs/code-mode-protocol` | Code Mode 协议 |
| `crates/connectors` | `codex-rs/connectors` | 连接器抽象 |
| `crates/exec-server-protocol` | `codex-rs/exec-server-protocol` | exec-server RPC 协议 |
| `crates/ext-items` | `codex-rs/ext/items` | 扩展事件 items |
| `crates/features` | `codex-rs/features` | feature flag |
| `crates/file-system` | `codex-rs/file-system` | 文件系统抽象 |
| `crates/git-utils` | `codex-rs/git-utils` | git 工具 |
| `crates/history` | `codex-rs/history` | 会话历史 |
| `crates/http-client` | `codex-rs/http-client` | HTTP 客户端工厂 |
| `crates/install-context` | `codex-rs/install-context` | 安装上下文 |
| `crates/keyring-store` | `codex-rs/keyring-store` | keyring 凭证存储 |
| `crates/login` | `codex-rs/login` | 登录/鉴权 |
| `crates/model-provider-info` | `codex-rs/model-provider-info` | 模型提供方元数据 |
| `crates/network-proxy` | `codex-rs/network-proxy` | 网络代理 |
| `crates/otel` | `codex-rs/otel` | OpenTelemetry 集成 |
| `crates/plugin` | `codex-rs/plugin` | 插件清单 |
| `crates/secrets` | `codex-rs/secrets` | 凭证管理 |
| `crates/shell-command` | `codex-rs/shell-command` | shell 命令建模 |
| `crates/terminal-detection` | `codex-rs/terminal-detection` | 终端检测 |
| `crates/utils-absolute-path` | `codex-rs/utils/absolute-path` | 绝对路径类型 |
| `crates/utils-cache` | `codex-rs/utils/cache` | 缓存 |
| `crates/utils-cargo-bin` | `codex-rs/utils/cargo-bin` | 测试用二进制定位 |
| `crates/utils-home-dir` | `codex-rs/utils/home-dir` | home 目录解析 |
| `crates/utils-image` | `codex-rs/utils/image` | 图片工具 |
| `crates/utils-output-truncation` | `codex-rs/utils/output-truncation` | 输出截断 |
| `crates/utils-path` | `codex-rs/utils/path-utils` | 路径工具 |
| `crates/utils-path-uri` | `codex-rs/utils/path-uri` | 路径 URI 编码 |
| `crates/utils-plugins` | `codex-rs/utils/plugins` | 插件工具 |
| `crates/utils-pty` | `codex-rs/utils/pty` | PTY |
| `crates/utils-rustls-provider` | `codex-rs/utils/rustls-provider` | rustls provider |
| `crates/utils-string` | `codex-rs/utils/string` | 字符串工具 |
| `crates/utils-template` | `codex-rs/utils/template` | 模板 |
| `crates/websocket-client` | `codex-rs/websocket-client` | WebSocket 客户端 |
| `crates/windows-sandbox-rs` | `codex-rs/windows-sandbox-rs` | Windows 受限令牌沙箱 |
| `crates/workload-identity` | `codex-rs/workload-identity` | 工作负载身份 |

同时从上游拷贝了以下**工作区级配置**（依赖版本表、lints、构建配置）：

- `Cargo.toml` 中的 `[workspace.dependencies]`（外部依赖版本）、`[workspace.lints]`、
  `[patch.crates-io]`、`[profile.*]`
- `rustfmt.toml`、`clippy.toml`、`.cargo/config.toml`

### 未拷贝部分（有意排除）

- `mcp-server`：依赖 `codex-core`（模型客户端与 Agent 会话循环，T3/T4 将以 AgentPlex
  自有实现替换），为满足"未捞取 codex 无关部分"的验收标准而延后接入。
- `tui`、`cli`、`app-server`、`core`、模型客户端等非执行层地基 crate。
- Bazel 构建文件（`BUILD.bazel` 等）。

### 相对上游的修改

除目录移动外，仅做了让 workspace 独立可构建的最小改动：

1. 根 `Cargo.toml` 重写：workspace 成员改为 `crates/*`，内部依赖以
   `{ path = "crates/..." }` 重新布线；`[patch.crates-io]` 移除上游注释掉的
   `crossterm` 补丁（本 workspace 无 crate 依赖 crossterm），保留
   `tokio-tungstenite` / `tungstenite` 两个 forks 补丁。
2. `crates/exec-server/Cargo.toml`：移除 dev-dependencies 中的
   `codex-exec-server-test-support`、`codex-test-binary-support`，并删除
   `tests/` 集成测试目录（其依赖链拖入 `codex-linux-sandbox` 等未拷贝 crate）。
3. `crates/login/Cargo.toml`：移除 dev-dependency `core_test_support`
   （位于上游 `core/tests/common`，会拖入 `codex-core`），并删除 `tests/` 目录；
   将 `skip_if_no_network!` 宏移植到 `login/src/test_support.rs` 以保留原有测试行为。
4. `crates/codex-api/src/files.rs`：测试
   `upload_openai_file_reports_blob_transport_diagnostics_without_sas` 内设置
   `NO_PROXY=127.0.0.1,localhost`。原因：reqwest 0.12 默认读取 Windows/macOS 系统
   代理，开启本地代理的机器上"连接死端口"的 PUT 会被代理以 502 应答，测试期望的
   传输层错误不会发生。此改动仅影响该测试进程，不改变生产代码行为。
5. `crates/exec-server/src/environment.rs`：测试
   `default_environment_has_ready_local_executor` 的 no-op 进程 argv 跨平台化
   （Unix `true` / Windows `cmd /c exit 0`）。上游仅在 Unix CI 上运行该测试；
   AgentPlex 以 Windows 为首要平台，须保证 Windows 本地可跑通。
6. `crates/engine`（AgentPlex 自有）：保持不变，继续作为执行引擎入口占位。
7. `crates/http-client/src/tls_backend_fallback.rs`（**生产代码**）：Windows Schannel
   把 TLS alert 70（protocol_version）同时映射为 `SEC_E_UNSUPPORTED_FUNCTION`
   （0x80090302）与 `SEC_E_INVALID_TOKEN`（0x80090326），且错误链上具体类型为
   `native_tls::Error`（Display/Debug 转发内部 `io::Error`，但 `source()` 不透出），
   `downcast_ref::<io::Error>()` 无法取得原始错误码。因此除原 raw_os_error 匹配外，
   增加对格式化消息 `(os error -2146893018)` / `0x80090326` 的匹配，使 rustls
   TLS 回退在 Windows 上按预期触发。对应的
   `crates/http-client/src/tls_backend_fallback_tests.rs` 增加了上述错误码的
   正/负用例。
8. `crates/http-client/src/route_aware_client_pool_tests.rs`（测试）：两处 Windows
   适配——测试 `without_url_redacts_transport_error_urls` 设置
   `NO_PROXY=127.0.0.1,localhost`（原因同第 4 条）；测试
   `reqwest_default_route_preserves_transport_redirects` 的内联 HTTP server 在
   accept 后将流设回阻塞模式（Windows 上 accept 的 socket 继承监听器的非阻塞
   模式，读取会以 WouldBlock 失败）。同文件 `spawn_response_server` 已有同样处理
   的部分保持不变。
9. `crates/network-proxy/src/lib.rs`（仅测试）：新增 `test_home` 模块，将
   `CODEX_HOME` 一次性重定向到进程级临时目录。原因：credential-broker / MITM CA
   配置会在真实用户 home（`~/.codex/proxy`）下落盘，开发机沙箱可能拒绝写入
   （os error 5）。`src/proxy.rs`、`src/runtime.rs`、`src/socks5.rs` 中会触发该落盘
   的测试同样在入口处调用 `ensure_test_codex_home()`。生产代码路径不受影响。
10. `crates/utils-pty/src/win/mod.rs`（生产代码）+ `src/tests.rs`、
    `src/windows_tests.rs`（测试）：新增 `conpty_available()` 运行时探测，真正创建
    一次 `RawConPty` 以确认环境支持 ConPTY（仅按 Windows 版本号判断不够——受限
    沙箱/远程会话可能版本够新但 `CreatePseudoConsole` 仍失败，如 HRESULT
    0x80070006）。依赖 ConPTY 的测试在探测失败时跳过。生产行为不变。
11. `crates/windows-sandbox-rs/src/env.rs`（生产代码）：`ensure_denybin` 支持
    `SBX_DENYBIN_DIR` 环境变量覆盖默认的 `~/.sbx-denybin`，便于测试与受限环境
    重定向 stub 目录；未设置时行为与上游一致。
12. `crates/windows-sandbox-rs`（仅测试）：
    - `src/env.rs` 新增 `test_support::ensure_test_denybin_dir()`，把
      `SBX_DENYBIN_DIR` 一次性重定向到进程级临时目录（原因同第 9 条：`~/.sbx-denybin`
      写入被沙箱拒绝）。
    - `src/unified_exec/tests.rs` 新增 `legacy_process_spawn_available()` 运行时探测：
      用受限令牌真实 spawn `cmd /c exit 0`，失败则跳过所有 legacy spawn 集成测试
      （受限环境会拦截 `CreateProcessAsUserW`，子进程立即以 0x80000005 退出）。
    - `src/unified_exec/tests.rs` 中 `elevated_non_tty_cmd_forwards_env_output_and_exit`
      标记 `#[ignore]`：elevated 后端需要已配置的沙箱用户 + command-runner helper +
      管理员权限，受限/CI 环境不可用（与文件内既有的两个 `legacy_tty_cmd` ignore 一致）。
13. `crates/shell-command/src/bash.rs`（生产代码）：`parse_plain_command_from_node` 的
    `if let Some(words) = ... { push } else { return None }` 按 clippy `question_mark`
    建议改写为 `let words = ...?; push`。该 lint 在本仓库所用 clippy 1.97 才出现
    （上游基线按旧 clippy 编写），改写后行为与语义完全等价。
14. `crates/otel/tests/suite/otlp_http_loopback.rs`（仅测试）：21 处 `assert!` 格式化
    参数中的冗余取址 `&body.chars().take(2000).collect::<String>()` 移除 `&`，按
    clippy `useless_borrows_in_formatting`（同样为 1.97 新 lint）修正，行为不变。

### 上游同步（drift 管理）

与上游 drift 需自行管理（ADR-0001）。重新同步时以上述 commit 为基线做 diff，
并保持本文件的"修改清单"同步更新。
