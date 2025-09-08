//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// List all manually banned IPs/Subnets.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::listbanned;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.listbanned().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// List all manually banned IPs/Subnets.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListbannedResponse(pub Vec<serde_json::Value>);

/// Calls the `listbanned` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listbanned(
    transport: &dyn TransportTrait,
) -> Result<ListbannedResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listbanned", &params).await?;
    Ok(serde_json::from_value::<ListbannedResponse>(raw)?)
}
