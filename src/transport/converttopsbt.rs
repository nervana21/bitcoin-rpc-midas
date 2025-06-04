//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// createpsbt and walletcreatefundedpsbt should be used for new applications.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::converttopsbt;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.converttopsbt(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `converttopsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ConverttopsbtResponse {
    /// The resulting raw transaction (base64-encoded string)
    pub field_0: String,
}



/// Calls the `converttopsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn converttopsbt(transport: &dyn Transport, hexstring: serde_json::Value, permitsigdata: serde_json::Value, iswitness: serde_json::Value) -> Result<ConverttopsbtResponse, TransportError> {
    let params = vec![json!(hexstring), json!(permitsigdata), json!(iswitness)];
    let raw = transport.send_request("converttopsbt", &params).await?;
    Ok(serde_json::from_value::<ConverttopsbtResponse>(raw)?)
}
