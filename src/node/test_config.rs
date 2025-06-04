use std::env;

/// TestConfig represents the configuration needed to run a Bitcoin node in a test environment.
/// This struct is the single source of truth for test‑node settings: RPC port, username, and password.
/// Defaults are:
/// - `rpc_port = 0` (auto‑select a free port)
/// - `rpc_username = "rpcuser"`
/// - `rpc_password = "rpcpassword"`
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// The port number for RPC communication with the Bitcoin node.
    /// A value of 0 indicates that an available port should be automatically selected.
    pub rpc_port: u16,
    /// The username for RPC authentication.
    /// Can be customized to match your `bitcoin.conf` `rpcuser` setting.
    pub rpc_username: String,
    /// The password for RPC authentication.
    /// Can be customized to match your `bitcoin.conf` `rpcpassword` setting.
    pub rpc_password: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            rpc_port: 0,
            rpc_username: "rpcuser".to_string(),
            rpc_password: "rpcpassword".to_string(),
        }
    }
}

impl TestConfig {
    /// Create a `TestConfig`, overriding defaults with environment variables:
    /// - `RPC_PORT`: overrides `rpc_port`
    /// - `RPC_USER`: overrides `rpc_username`
    /// - `RPC_PASS`: overrides `rpc_password`
    pub fn from_env() -> Self {
        let mut cfg = Self::default();
        if let Ok(port_str) = env::var("RPC_PORT") {
            if let Ok(port) = port_str.parse() {
                cfg.rpc_port = port;
            }
        }
        if let Ok(user) = env::var("RPC_USER") {
            cfg.rpc_username = user;
        }
        if let Ok(pass) = env::var("RPC_PASS") {
            cfg.rpc_password = pass;
        }
        cfg
    }
}