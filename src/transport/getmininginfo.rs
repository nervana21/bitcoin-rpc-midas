//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns a json object containing mining-related information.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getmininginfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getmininginfo().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns a json object containing mining-related information.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmininginfoResponse {
    pub blocks: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentblockweight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentblocktx: Option<f64>,
    pub difficulty: f64,
    pub networkhashps: f64,
    pub pooledtx: f64,
    pub chain: String,
    pub warnings: Vec<serde_json::Value>,
}



/// Calls the `getmininginfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmininginfo(transport: &dyn TransportTrait) -> Result<GetmininginfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getmininginfo", &params).await?;
    Ok(serde_json::from_value::<GetmininginfoResponse>(raw)?)
}
