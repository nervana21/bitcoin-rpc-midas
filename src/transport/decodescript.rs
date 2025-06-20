//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Decode a hex-encoded script.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::decodescript;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.decodescript(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Decode a hex-encoded script.
#[derive(Debug, Deserialize, Serialize)]
pub struct DecodescriptResponse {
    pub asm: String,
    pub desc: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2sh: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segwit: Option<serde_json::Value>,
}



/// Calls the `decodescript` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn decodescript(transport: &dyn TransportTrait, hexstring: serde_json::Value) -> Result<DecodescriptResponse, TransportError> {
    let params = vec![json!(hexstring)];
    let raw = transport.send_request("decodescript", &params).await?;
    Ok(serde_json::from_value::<DecodescriptResponse>(raw)?)
}
