//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::generateblock;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.generateblock(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateblockResponse {
    pub hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

/// Calls the `generateblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn generateblock(
    transport: &dyn TransportTrait,
    output: serde_json::Value,
    transactions: serde_json::Value,
    submit: serde_json::Value,
) -> Result<GenerateblockResponse, TransportError> {
    let params = vec![json!(output), json!(transactions), json!(submit)];
    let raw = transport.send_request("generateblock", &params).await?;
    Ok(serde_json::from_value::<GenerateblockResponse>(raw)?)
}
