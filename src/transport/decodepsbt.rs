//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::decodepsbt;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.decodepsbt(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
#[derive(Debug, Deserialize, Serialize)]
pub struct DecodepsbtResponse {
    pub tx: serde_json::Value,
    pub global_xpubs: Vec<serde_json::Value>,
    pub psbt_version: u32,
    pub proprietary: Vec<serde_json::Value>,
    pub unknown: serde_json::Value,
    pub inputs: Vec<serde_json::Value>,
    pub outputs: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
}

/// Calls the `decodepsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn decodepsbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
) -> Result<DecodepsbtResponse, TransportError> {
    let params = vec![json!(psbt)];
    let raw = transport.send_request("decodepsbt", &params).await?;
    Ok(serde_json::from_value::<DecodepsbtResponse>(raw)?)
}
