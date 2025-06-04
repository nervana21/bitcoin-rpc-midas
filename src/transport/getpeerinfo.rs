//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns data about each connected network peer as a json array of objects.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getpeerinfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getpeerinfo().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getpeerinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetpeerinfoResponse {
    pub field_0: Vec<serde_json::Value>,
}



/// Calls the `getpeerinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getpeerinfo(transport: &dyn Transport) -> Result<GetpeerinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getpeerinfo", &params).await?;
    Ok(serde_json::from_value::<GetpeerinfoResponse>(raw)?)
}
