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
use transport::{TransportTrait, TransportError};
/// Returns data about each connected network peer as a json array of objects.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetpeerinfoResponse(pub Vec<serde_json::Value>);



/// Calls the `getpeerinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getpeerinfo(transport: &dyn TransportTrait) -> Result<GetpeerinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getpeerinfo", &params).await?;
    Ok(serde_json::from_value::<GetpeerinfoResponse>(raw)?)
}
