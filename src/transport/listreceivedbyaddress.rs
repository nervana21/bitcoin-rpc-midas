//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// List balances by receiving address.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::listreceivedbyaddress;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.listreceivedbyaddress(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `listreceivedbyaddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListreceivedbyaddressResponse {
    pub field_0: Vec<serde_json::Value>,
}



/// Calls the `listreceivedbyaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listreceivedbyaddress(transport: &dyn Transport, minconf: serde_json::Value, include_empty: serde_json::Value, include_watchonly: serde_json::Value, address_filter: serde_json::Value, include_immature_coinbase: serde_json::Value) -> Result<ListreceivedbyaddressResponse, TransportError> {
    let params = vec![json!(minconf), json!(include_empty), json!(include_watchonly), json!(address_filter), json!(include_immature_coinbase)];
    let raw = transport.send_request("listreceivedbyaddress", &params).await?;
    Ok(serde_json::from_value::<ListreceivedbyaddressResponse>(raw)?)
}
