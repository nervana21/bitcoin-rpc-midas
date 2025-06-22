//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// List received transactions by label.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::listreceivedbylabel;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.listreceivedbylabel(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// List received transactions by label.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListreceivedbylabelResponse(pub Vec<serde_json::Value>);



/// Calls the `listreceivedbylabel` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listreceivedbylabel(transport: &dyn TransportTrait, minconf: serde_json::Value, include_empty: serde_json::Value, include_watchonly: serde_json::Value, include_immature_coinbase: serde_json::Value) -> Result<ListreceivedbylabelResponse, TransportError> {
    let params = vec![json!(minconf), json!(include_empty), json!(include_watchonly), json!(include_immature_coinbase)];
    let raw = transport.send_request("listreceivedbylabel", &params).await?;
    Ok(serde_json::from_value::<ListreceivedbylabelResponse>(raw)?)
}
