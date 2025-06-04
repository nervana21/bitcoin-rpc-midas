//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::listlabels;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.listlabels(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `listlabels` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListlabelsResponse {
    pub field_0: Vec<serde_json::Value>,
}



/// Calls the `listlabels` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listlabels(transport: &dyn Transport, purpose: serde_json::Value) -> Result<ListlabelsResponse, TransportError> {
    let params = vec![json!(purpose)];
    let raw = transport.send_request("listlabels", &params).await?;
    Ok(serde_json::from_value::<ListlabelsResponse>(raw)?)
}
