//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::savemempool;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.savemempool().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
#[derive(Debug, Deserialize, Serialize)]
pub struct SavemempoolResponse {
    pub filename: String,
}

/// Calls the `savemempool` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn savemempool(
    transport: &dyn TransportTrait,
) -> Result<SavemempoolResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("savemempool", &params).await?;
    Ok(serde_json::from_value::<SavemempoolResponse>(raw)?)
}
