//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::decodepsbt;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.decodepsbt(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `decodepsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecodepsbtResponse {
    /// The decoded network-serialized unsigned transaction.
    pub tx: serde_json::Value,
    pub global_xpubs: Vec<serde_json::Value>,
    /// The PSBT version number. Not to be confused with the unsigned transaction version
    pub psbt_version: u64,
    /// The global proprietary map
    pub proprietary: Vec<serde_json::Value>,
    /// The unknown global fields
    pub unknown: serde_json::Value,
    pub inputs: Vec<serde_json::Value>,
    pub outputs: Vec<serde_json::Value>,
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<bitcoin::Amount>,
}



/// Calls the `decodepsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn decodepsbt(transport: &dyn Transport, psbt: serde_json::Value) -> Result<DecodepsbtResponse, TransportError> {
    let params = vec![json!(psbt)];
    let raw = transport.send_request("decodepsbt", &params).await?;
    Ok(serde_json::from_value::<DecodepsbtResponse>(raw)?)
}
