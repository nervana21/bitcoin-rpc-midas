//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Creates a multi-signature address with n signature of m keys required.
/// It returns a json object with the address and redeemScript.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::createmultisig;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.createmultisig(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `createmultisig` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatemultisigResponse {
    /// The value of the new multisig address.
    pub address: String,
    /// The string value of the hex-encoded redemption script.
    pub redeem_script: String,
    /// The descriptor for this multisig
    pub descriptor: String,
    /// Any warnings resulting from the creation of this multisig
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}



/// Calls the `createmultisig` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn createmultisig(transport: &dyn Transport, nrequired: serde_json::Value, keys: serde_json::Value, address_type: serde_json::Value) -> Result<CreatemultisigResponse, TransportError> {
    let params = vec![json!(nrequired), json!(keys), json!(address_type)];
    let raw = transport.send_request("createmultisig", &params).await?;
    Ok(serde_json::from_value::<CreatemultisigResponse>(raw)?)
}
