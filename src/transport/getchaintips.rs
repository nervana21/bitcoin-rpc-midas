//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getchaintips;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getchaintips().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetchaintipsResponse(pub Vec<serde_json::Value>);

/// Calls the `getchaintips` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getchaintips(
    transport: &dyn TransportTrait,
) -> Result<GetchaintipsResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getchaintips", &params).await?;
    Ok(serde_json::from_value::<GetchaintipsResponse>(raw)?)
}
