//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// List all manually banned IPs/Subnets.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::listbanned;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.listbanned().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// List all manually banned IPs/Subnets.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListbannedResponse(pub Vec<serde_json::Value>);



/// Calls the `listbanned` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listbanned(transport: &dyn TransportTrait) -> Result<ListbannedResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listbanned", &params).await?;
    Ok(serde_json::from_value::<ListbannedResponse>(raw)?)
}
