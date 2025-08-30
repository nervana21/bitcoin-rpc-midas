use bitcoin::Network;
use std::env;

/// TestConfig represents the configuration needed to run a Bitcoin node in a test environment.
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// The port number for RPC communication with the Bitcoin node.
    /// A value of 0 indicates that an available port should be automatically selected.
    pub rpc_port: u16,
    /// The username for RPC authentication.
    pub rpc_username: String,
    /// The password for RPC authentication.
    pub rpc_password: String,
    /// Which Bitcoin network to run against.
    pub network: Network,
    /// Bitcoin Core version; `None` to auto-detect
    pub core_version: Option<u32>,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            rpc_port: 0,
            rpc_username: "rpcuser".to_string(),
            rpc_password: "rpcpassword".to_string(),
            network: Network::Regtest,
            core_version: Some(29), // TODO: remove this once we have a way to auto-detect the version
        }
    }
}

impl TestConfig {
    /// Return the value used with `-chain=<value>` for the configured network
    pub fn as_chain_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match self.network {
            Network::Bitcoin => "main",
            Network::Regtest => "regtest",
            Network::Signet => "signet",
            Network::Testnet => "testnet",
            Network::Testnet4 => "testnet4",
            _ => panic!("Unsupported network variant"),
        }
    }

    /// Parse network from common strings (case-insensitive). Accepts: regtest, testnet|test,
    /// signet, mainnet|main|bitcoin, testnet4.
    pub fn network_from_str(s: &str) -> Option<Network> {
        match s.to_ascii_lowercase().as_str() {
            "regtest" => Some(Network::Regtest),
            "testnet" | "test" => Some(Network::Testnet),
            "signet" => Some(Network::Signet),
            "mainnet" | "main" | "bitcoin" => Some(Network::Bitcoin),
            "testnet4" => Some(Network::Testnet4),
            _ => None,
        }
    }

    /// Create a `TestConfig`, overriding defaults with environment variables:
    /// - `RPC_PORT`: overrides `rpc_port`
    /// - `RPC_USER`: overrides `rpc_username`
    /// - `RPC_PASS`: overrides `rpc_password`
    /// - `RPC_NETWORK`: one of `regtest`, `testnet|test`, `signet`, `mainnet|main|bitcoin`, `testnet4`
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
        if let Ok(net) = env::var("RPC_NETWORK") {
            if let Some(n) = Self::network_from_str(&net) {
                cfg.network = n;
            }
        }
        cfg
    }
}