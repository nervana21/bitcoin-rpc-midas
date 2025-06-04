use anyhow::Result;
use async_trait::async_trait;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tempfile::TempDir;
use tokio::io::AsyncBufReadExt;
use tokio::process::{Child, Command};
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, error, info, warn};
use std::process::Stdio;

use super::test_config::TestConfig;

/// Represents the state of a Bitcoin node
#[derive(Debug, Clone)]
pub struct NodeState {
    pub is_running: bool,
    pub version: String,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            is_running: false,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Trait defining the interface for a Bitcoin node manager
#[async_trait]
pub trait NodeManager: Send + Sync + std::any::Any + std::fmt::Debug {
    async fn start(&self) -> Result<()>;
    async fn stop(&mut self) -> Result<()>;
    async fn get_state(&self) -> Result<NodeState>;
    /// Return the RPC port this manager was configured with
    fn rpc_port(&self) -> u16;
}

/// Implementation of the Bitcoin node manager
#[derive(Debug)]
pub struct BitcoinNodeManager {
    state: Arc<RwLock<NodeState>>,
    child: Arc<Mutex<Option<Child>>>,
    pub(crate) rpc_port: u16,
    rpc_username: String,
    rpc_password: String,
    _datadir: Option<TempDir>,
}

impl BitcoinNodeManager {
    pub fn new() -> Result<Self> {
        Self::new_with_config(&TestConfig::default())
    }

    pub fn new_with_config(config: &TestConfig) -> Result<Self> {
        let datadir = TempDir::new()?;

        // Handle automatic port selection:
        // When rpc_port is 0, we need to find an available port dynamically.
        let rpc_port = if config.rpc_port == 0 {
            // Bind to port 0 to let the OS assign an available port
            let listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
            listener.local_addr()?.port()
        } else {
            config.rpc_port
        };

        Ok(Self {
            state: Arc::new(RwLock::new(NodeState::default())),
            child: Arc::new(Mutex::new(None)),
            rpc_port,
            rpc_username: config.rpc_username.clone(),
            rpc_password: config.rpc_password.clone(),
            _datadir: Some(datadir),
        })
    }

    pub(crate) async fn start_internal(&self) -> Result<()> {
        let mut state = self.state.write().await;
        if state.is_running {
            info!("[DEBUG] Node is already running, skipping start");
            return Ok(());
        }

        info!(
            "[DEBUG] Starting Bitcoin node with datadir: {:?}",
            self._datadir.as_ref().unwrap().path()
        );
        let datadir = self._datadir.as_ref().unwrap().path();
        let mut cmd = Command::new("bitcoind");
        cmd.args([
            "-regtest",
            "-listen=0",
            &format!("-datadir={}", datadir.display()),
            &format!("-rpcport={}", self.rpc_port),
            &format!("-rpcbind=127.0.0.1:{}", self.rpc_port),
            "-rpcallowip=127.0.0.1",
            "-fallbackfee=0.0002",
            "-server=1",
            "-prune=1",
            &format!("-rpcuser={}", self.rpc_username),
            &format!("-rpcpassword={}", self.rpc_password),
        ]);

        // Capture both stdout and stderr for better error reporting
        cmd.stderr(Stdio::piped());
        cmd.stdout(Stdio::piped());

        info!("[DEBUG] Spawning bitcoind process");
        let mut child = cmd.spawn()?;
        info!("[DEBUG] bitcoind process spawned successfully");

        // Read stderr in a separate task
        let stderr = child.stderr.take().unwrap();
        let stderr_reader = tokio::io::BufReader::new(stderr);
        tokio::spawn(async move {
            let mut lines = stderr_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                error!("[DEBUG] bitcoind stderr: {}", line);
            }
        });

        // Read stdout in a separate task
        let stdout = child.stdout.take().unwrap();
        let stdout_reader = tokio::io::BufReader::new(stdout);
        tokio::spawn(async move {
            let mut lines = stdout_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                info!("[DEBUG] bitcoind stdout: {}", line);
            }
        });

        // Store the child process
        info!("[DEBUG] Storing child process");
        let mut child_guard = self.child.lock().await;
        *child_guard = Some(child);
        info!("[DEBUG] Child process stored successfully");

        // Wait for node to be ready
        info!("[DEBUG] Waiting for node to be ready");
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut attempts = 0;
        while Instant::now() < deadline {
            if let Some(child) = child_guard.as_mut() {
                if let Ok(Some(status)) = child.try_wait() {
                    let error = format!("Bitcoin node exited early with status: {}", status);
                    error!("[DEBUG] {}", error);
                    anyhow::bail!(error);
                }
            }

            // Try to connect to RPC
            info!(
                "[DEBUG] Attempt {}: Trying to connect to RPC at port {}",
                attempts + 1,
                self.rpc_port
            );
            let client = reqwest::Client::new();
            match client
                .post(format!("http://127.0.0.1:{}/", self.rpc_port))
                .basic_auth(&self.rpc_username, Some(&self.rpc_password))
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "getnetworkinfo",
                    "params": [],
                    "id": 1
                }))
                .send()
                .await
            {
                Ok(response) => {
                    info!("[DEBUG] RPC response status: {}", response.status());
                    if response.status().is_success() {
                        state.is_running = true;
                        info!(
                            "[DEBUG] Bitcoin node started successfully on port {}",
                            self.rpc_port
                        );
                        return Ok(());
                    } else {
                        warn!(
                            "[DEBUG] RPC request failed with status {} (attempt {})",
                            response.status(),
                            attempts
                        );
                    }
                }
                Err(e) => {
                    debug!(
                        "[DEBUG] Failed to connect to RPC (attempt {}): {}",
                        attempts, e
                    );
                }
            }

            attempts += 1;
            info!("[DEBUG] Waiting 200ms before next attempt");
            tokio::time::sleep(Duration::from_millis(200)).await;
        }

        let error = format!(
            "[DEBUG] Timed out waiting for Bitcoin node to start on port {} after {} attempts",
            self.rpc_port, attempts
        );
        error!("{}", error);
        anyhow::bail!(error);
    }

    pub(crate) async fn stop_internal(&mut self) -> Result<()> {
        let mut state = self.state.write().await;
        if !state.is_running {
            return Ok(());
        }

        let child = self.child.lock().await.take();
        if let Some(mut child) = child {
            std::mem::drop(child.kill());
            std::mem::drop(child.wait());
        }

        state.is_running = false;
        Ok(())
    }

    async fn get_state(&self) -> Result<NodeState> {
        Ok(self.state.read().await.clone())
    }

    fn rpc_port(&self) -> u16 {
        self.rpc_port
    }
}

#[async_trait]
impl NodeManager for BitcoinNodeManager {
    async fn start(&self) -> Result<()> {
        self.start_internal().await
    }

    async fn stop(&mut self) -> Result<()> {
        self.stop_internal().await
    }

    async fn get_state(&self) -> Result<NodeState> {
        self.get_state().await
    }

    fn rpc_port(&self) -> u16 {
        self.rpc_port()
    }
}

impl Drop for BitcoinNodeManager {
    fn drop(&mut self) {
        if let Some(mut child) = self
            .child
            .try_lock()
            .ok()
            .and_then(|mut guard| guard.take())
        {
            std::mem::drop(child.kill());
            std::mem::drop(child.wait());
        }
    }
}

impl Default for BitcoinNodeManager {
    fn default() -> Self {
        Self::new_with_config(&TestConfig::default())
            .expect("Failed to create default BitcoinNodeManager")
    }
}