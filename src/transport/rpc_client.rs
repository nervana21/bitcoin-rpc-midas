use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
use std::fmt;
use crate::transport::{TransportTrait, TransportError, DefaultTransport, BatchBuilder};

/// Thin wrapper around a transport for making RPC calls
pub struct RpcClient {
    transport: Arc<dyn TransportTrait>,
}

impl fmt::Debug for RpcClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RpcClient")
            .field("transport", &"<dyn TransportTrait>")
            .finish()
    }
}

impl RpcClient {
    /// Wrap an existing transport (no URL+auth dance)
    pub fn from_transport(inner: Arc<dyn TransportTrait>) -> Self {
        Self { transport: inner }
    }

    /// Create a new RPC client with URL and auth
    pub fn new(url: &str, user: &str, pass: &str) -> Self {
        let transport = DefaultTransport::new(
            url.to_string(),
            Some((user.to_string(), pass.to_string())),
        );
        Self { transport: Arc::new(transport) }
    }

    /// Call a JSON-RPC method
    pub async fn call_method(&self, method: &str, params: &[Value]) -> Result<Value, TransportError> {
        self.transport.send_request(method, params).await
    }

    /// Start building a batch of RPC calls
    pub fn batch(&self) -> BatchBuilder {
        BatchBuilder::new(self.transport.clone())
    }
}