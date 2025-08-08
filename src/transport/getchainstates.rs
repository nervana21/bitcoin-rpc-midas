//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Return information about chainstates.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getchainstates;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getchainstates().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Return information about chainstates.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetchainstatesResponse {
    pub headers: u64,
    pub chainstates: Vec<serde_json::Value>,
}

/// Calls the `getchainstates` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getchainstates(
    transport: &dyn TransportTrait,
) -> Result<GetchainstatesResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getchainstates", &params).await?;
    Ok(serde_json::from_value::<GetchainstatesResponse>(raw)?)
}
