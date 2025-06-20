//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns the total uptime of the server.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::uptime;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.uptime().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns the total uptime of the server.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UptimeResponse(pub f64);



/// Calls the `uptime` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn uptime(transport: &dyn TransportTrait) -> Result<UptimeResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("uptime", &params).await?;
    Ok(serde_json::from_value::<UptimeResponse>(raw)?)
}
