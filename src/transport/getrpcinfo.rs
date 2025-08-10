//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Returns details of the RPC server.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::getrpcinfo;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getrpcinfo().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns details of the RPC server.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetrpcinfoResponse {
    pub active_commands: Vec<serde_json::Value>,
    pub logpath: String,
}

/// Calls the `getrpcinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrpcinfo(
    transport: &dyn TransportTrait,
) -> Result<GetrpcinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getrpcinfo", &params).await?;
    Ok(serde_json::from_value::<GetrpcinfoResponse>(raw)?)
}
