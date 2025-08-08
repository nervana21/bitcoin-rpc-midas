//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Creates a multi-signature address with n signatures of m keys required.
/// It returns a json object with the address and redeemScript.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::createmultisig;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.createmultisig(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Creates a multi-signature address with n signatures of m keys required.
/// It returns a json object with the address and redeemScript.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatemultisigResponse {
    pub address: String,
    pub redeem_script: String,
    pub descriptor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}

/// Calls the `createmultisig` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn createmultisig(
    transport: &dyn TransportTrait,
    nrequired: serde_json::Value,
    keys: serde_json::Value,
    address_type: serde_json::Value,
) -> Result<CreatemultisigResponse, TransportError> {
    let params = vec![json!(nrequired), json!(keys), json!(address_type)];
    let raw = transport.send_request("createmultisig", &params).await?;
    Ok(serde_json::from_value::<CreatemultisigResponse>(raw)?)
}
