//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Returns the total uptime of the server.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::uptime;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.uptime().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns the total uptime of the server.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UptimeResponse(pub u64);

/// Calls the `uptime` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn uptime(transport: &dyn TransportTrait) -> Result<UptimeResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("uptime", &params).await?;
    Ok(serde_json::from_value::<UptimeResponse>(raw)?)
}
