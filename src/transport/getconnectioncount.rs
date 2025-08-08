//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns the number of connections to other nodes.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getconnectioncount;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getconnectioncount().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns the number of connections to other nodes.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetconnectioncountResponse(pub u64);

/// Calls the `getconnectioncount` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getconnectioncount(
    transport: &dyn TransportTrait,
) -> Result<GetconnectioncountResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport
        .send_request("getconnectioncount", &params)
        .await?;
    Ok(serde_json::from_value::<GetconnectioncountResponse>(raw)?)
}
