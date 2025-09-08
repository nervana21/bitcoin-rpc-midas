//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Returns a json object containing mining-related information.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getmininginfo;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getmininginfo().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns a json object containing mining-related information.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmininginfoResponse {
    pub blocks: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentblockweight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentblocktx: Option<u64>,
    pub bits: String,
    pub difficulty: f64,
    pub target: String,
    pub networkhashps: u64,
    pub pooledtx: u64,
    pub blockmintxfee: f64,
    pub chain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signet_challenge: Option<String>,
    pub next: serde_json::Value,
    pub warnings: Vec<serde_json::Value>,
}

/// Calls the `getmininginfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmininginfo(
    transport: &dyn TransportTrait,
) -> Result<GetmininginfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getmininginfo", &params).await?;
    Ok(serde_json::from_value::<GetmininginfoResponse>(raw)?)
}
