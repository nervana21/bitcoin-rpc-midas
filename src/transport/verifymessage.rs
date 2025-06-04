//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Verify a signed message.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::verifymessage;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.verifymessage(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `verifymessage` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifymessageResponse {
    /// If the signature is verified or not.
    pub field_0: bool,
}



/// Calls the `verifymessage` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn verifymessage(transport: &dyn Transport, address: serde_json::Value, signature: serde_json::Value, message: serde_json::Value) -> Result<VerifymessageResponse, TransportError> {
    let params = vec![json!(address), json!(signature), json!(message)];
    let raw = transport.send_request("verifymessage", &params).await?;
    Ok(serde_json::from_value::<VerifymessageResponse>(raw)?)
}
