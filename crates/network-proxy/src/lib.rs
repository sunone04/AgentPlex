#![deny(clippy::print_stdout, clippy::print_stderr)]

mod attribution;
mod authorization_path;
mod certs;
mod config;
mod connect_policy;
mod credential_broker;
mod environment_policy;
mod http_proxy;
mod mitm;
mod mitm_hook;
mod native_certs;
mod network_policy;
mod policy;
mod proxy;
mod reasons;
mod remote_config;
mod responses;
mod runtime;
mod socks5;
mod state;
mod upstream;
#[cfg(target_os = "windows")]
mod windows_proxy_ingress;
#[cfg(target_os = "windows")]
mod windows_tcp_attribution;

pub use attribution::PROXY_ATTRIBUTION_TOKEN_ENV_KEY;
pub use attribution::write_attribution_frame;
pub use certs::CUSTOM_CA_ENV_KEYS;
pub use certs::is_managed_mitm_ca_trust_bundle_path;
pub use config::NetworkDomainPermission;
pub use config::NetworkDomainPermissionEntry;
pub use config::NetworkDomainPermissions;
pub use config::NetworkMode;
pub use config::NetworkProxyConfig;
pub use config::NetworkUnixSocketPermission;
pub use config::NetworkUnixSocketPermissions;
pub use config::host_and_port_from_network_addr;
pub use config::managed_proxy_ports;
pub use credential_broker::CREDENTIAL_BROKER_ACTIVE_ENV_KEY;
pub use credential_broker::brokered_credential_dummy_env_keys;
pub use credential_broker::brokered_credential_env_keys;
pub use environment_policy::EnvironmentNetworkPolicy;
pub use mitm_hook::InjectedHeaderConfig;
pub use mitm_hook::MitmHookActionsConfig;
pub use mitm_hook::MitmHookBodyConfig;
pub use mitm_hook::MitmHookConfig;
pub use mitm_hook::MitmHookMatchConfig;
pub use network_policy::NetworkDecision;
pub use network_policy::NetworkDecisionSource;
pub use network_policy::NetworkPolicyAuditEvent;
pub use network_policy::NetworkPolicyAuditObserver;
pub use network_policy::NetworkPolicyDecider;
pub use network_policy::NetworkPolicyDeciderFuture;
pub use network_policy::NetworkPolicyDecision;
pub use network_policy::NetworkPolicyRequest;
pub use network_policy::NetworkPolicyRequestArgs;
pub use network_policy::NetworkProtocol;
pub use policy::normalize_host;
pub use proxy::ALL_PROXY_ENV_KEYS;
pub use proxy::ALLOW_LOCAL_BINDING_ENV_KEY;
pub use proxy::Args;
#[cfg(target_os = "macos")]
pub use proxy::CODEX_PROXY_GIT_SSH_COMMAND_MARKER;
pub use proxy::DEFAULT_NO_PROXY_VALUE;
pub use proxy::ManagedNetworkSandboxContext;
pub use proxy::NO_PROXY_ENV_KEYS;
pub use proxy::NetworkProxy;
pub use proxy::NetworkProxyBuilder;
pub use proxy::NetworkProxyHandle;
pub use proxy::PROXY_ACTIVE_ENV_KEY;
pub use proxy::PROXY_ENV_KEYS;
#[cfg(target_os = "macos")]
pub use proxy::PROXY_GIT_SSH_COMMAND_ENV_KEY;
pub use proxy::PROXY_URL_ENV_KEYS;
pub use proxy::PreparedManagedNetwork;
pub use proxy::has_proxy_url_env_vars;
pub use proxy::is_managed_proxy_env_var;
pub use proxy::proxy_url_env_value;
pub use proxy::strip_managed_proxy_env;
pub use remote_config::RemoteNetworkProxyConfig;
pub use remote_config::RemoteNetworkProxyLaunchConfig;
pub use runtime::BlockedRequest;
pub use runtime::BlockedRequestArgs;
pub use runtime::BlockedRequestObserver;
pub use runtime::BlockedRequestObserverFuture;
pub use runtime::ConfigReloader;
pub use runtime::ConfigReloaderFuture;
pub use runtime::ConfigState;
pub use runtime::NetworkProxyState;
pub use state::NetworkProxyAuditMetadata;
pub use state::NetworkProxyConstraintError;
pub use state::NetworkProxyConstraints;
pub use state::PartialNetworkProxyConfig;
pub use state::build_config_state;
pub use state::validate_policy_against_constraints;

/// Test-only redirection of the Codex home directory.
///
/// Credential-broker and managed-MITM-CA configuration materializes directories under
/// `CODEX_HOME` (defaulting to the real user home). Tests must not touch the real home: it may
/// be read-only or sandboxed on developer machines, and polluting it with proxy artifacts is
/// undesirable anyway. Tests that reach `build_config_state` with the credential broker or
/// MITM enabled call [`test_home::ensure_test_codex_home`] first, which points `CODEX_HOME`
/// at a per-process temporary directory exactly once.
#[cfg(test)]
pub(crate) mod test_home {
    use std::sync::OnceLock;

    pub(crate) fn ensure_test_codex_home() {
        static INIT: OnceLock<()> = OnceLock::new();
        INIT.get_or_init(|| {
            if std::env::var("CODEX_HOME").is_ok() {
                return;
            }
            let dir = std::env::temp_dir().join(format!(
                "agentplex-network-proxy-tests-{}",
                std::process::id()
            ));
            std::fs::create_dir_all(&dir).expect("create test CODEX_HOME");
            // SAFETY: `OnceLock` serializes this initialization and every caller invokes this
            // helper before resolving the Codex home, so no test reads CODEX_HOME concurrently.
            unsafe { std::env::set_var("CODEX_HOME", &dir) };
        });
    }
}
