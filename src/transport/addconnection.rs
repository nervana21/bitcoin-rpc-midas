//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Open an outbound connection to a specified node. This RPC is for testing only.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::addconnection;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.addconnection(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Open an outbound connection to a specified node. This RPC is for testing only.
#[derive(Debug, Deserialize, Serialize)]
pub struct AddconnectionResponse {
    pub address: String,
    pub connection_type: String,
}

/// Calls the `addconnection` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn addconnection(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    connection_type: serde_json::Value,
    v2transport: serde_json::Value,
) -> Result<AddconnectionResponse, TransportError> {
    let params = vec![json!(address), json!(connection_type), json!(v2transport)];
    let raw = transport.send_request("addconnection", &params).await?;
    Ok(serde_json::from_value::<AddconnectionResponse>(raw)?)
}
