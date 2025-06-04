//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::createpsbt;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.createpsbt(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `createpsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatepsbtResponse {
    /// The resulting raw transaction (base64-encoded string)
    pub field_0: String,
}



/// Calls the `createpsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn createpsbt(transport: &dyn Transport, inputs: serde_json::Value, outputs: serde_json::Value, locktime: serde_json::Value, replaceable: serde_json::Value) -> Result<CreatepsbtResponse, TransportError> {
    let params = vec![json!(inputs), json!(outputs), json!(locktime), json!(replaceable)];
    let raw = transport.send_request("createpsbt", &params).await?;
    Ok(serde_json::from_value::<CreatepsbtResponse>(raw)?)
}
