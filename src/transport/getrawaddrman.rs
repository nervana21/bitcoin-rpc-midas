//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Returns information on all address manager entries for the new and tried tables.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::getrawaddrman;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getrawaddrman().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Returns information on all address manager entries for the new and tried tables.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetrawaddrmanResponse {
    pub table: serde_json::Value,
}

/// Calls the `getrawaddrman` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrawaddrman(
    transport: &dyn TransportTrait,
) -> Result<GetrawaddrmanResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getrawaddrman", &params).await?;
    Ok(serde_json::from_value::<GetrawaddrmanResponse>(raw)?)
}
