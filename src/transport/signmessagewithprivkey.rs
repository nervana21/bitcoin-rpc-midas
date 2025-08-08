//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Sign a message with the private key of an address

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::signmessagewithprivkey;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.signmessagewithprivkey(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Sign a message with the private key of an address
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignmessagewithprivkeyResponse(pub String);

/// Calls the `signmessagewithprivkey` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn signmessagewithprivkey(
    transport: &dyn TransportTrait,
    privkey: serde_json::Value,
    message: serde_json::Value,
) -> Result<SignmessagewithprivkeyResponse, TransportError> {
    let params = vec![json!(privkey), json!(message)];
    let raw = transport
        .send_request("signmessagewithprivkey", &params)
        .await?;
    Ok(serde_json::from_value::<SignmessagewithprivkeyResponse>(
        raw,
    )?)
}
