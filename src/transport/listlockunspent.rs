//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Returns list of temporarily unspendable outputs.
/// See the lockunspent call to lock and unlock transactions for spending.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::listlockunspent;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.listlockunspent().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns list of temporarily unspendable outputs.
/// See the lockunspent call to lock and unlock transactions for spending.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListlockunspentResponse(pub Vec<serde_json::Value>);

/// Calls the `listlockunspent` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listlockunspent(
    transport: &dyn TransportTrait,
) -> Result<ListlockunspentResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listlockunspent", &params).await?;
    Ok(serde_json::from_value::<ListlockunspentResponse>(raw)?)
}
