//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Return a JSON object representing the serialized, hex-encoded transaction.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::decoderawtransaction;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.decoderawtransaction(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Return a JSON object representing the serialized, hex-encoded transaction.
#[derive(Debug, Deserialize, Serialize)]
pub struct DecoderawtransactionResponse {
    pub txid: bitcoin::Txid,
    pub hash: String,
    pub size: u64,
    pub vsize: u64,
    pub weight: f64,
    pub version: u32,
    pub locktime: serde_json::Value,
    pub vin: Vec<serde_json::Value>,
    pub vout: Vec<serde_json::Value>,
}



/// Calls the `decoderawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn decoderawtransaction(transport: &dyn TransportTrait, hexstring: serde_json::Value, iswitness: serde_json::Value) -> Result<DecoderawtransactionResponse, TransportError> {
    let params = vec![json!(hexstring), json!(iswitness)];
    let raw = transport.send_request("decoderawtransaction", &params).await?;
    Ok(serde_json::from_value::<DecoderawtransactionResponse>(raw)?)
}
