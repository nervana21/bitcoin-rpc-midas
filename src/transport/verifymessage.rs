//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Verify a signed message.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::verifymessage;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.verifymessage(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Verify a signed message.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VerifymessageResponse(pub bool);

/// Calls the `verifymessage` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn verifymessage(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    signature: serde_json::Value,
    message: serde_json::Value,
) -> Result<VerifymessageResponse, TransportError> {
    let params = vec![json!(address), json!(signature), json!(message)];
    let raw = transport.send_request("verifymessage", &params).await?;
    Ok(serde_json::from_value::<VerifymessageResponse>(raw)?)
}
