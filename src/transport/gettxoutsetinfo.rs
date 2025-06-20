//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::gettxoutsetinfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.gettxoutsetinfo(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns statistics about the unspent transaction output set.
    /// Note this call may take some time if you are not using coinstatsindex.
#[derive(Debug, Deserialize, Serialize)]
pub struct GettxoutsetinfoResponse {
    pub height: u64,
    pub bestblock: String,
    pub txouts: f64,
    pub bogosize: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_serialized_3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muhash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<u64>,
    pub total_amount: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_unspendable_amount: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_info: Option<serde_json::Value>,
}



/// Calls the `gettxoutsetinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettxoutsetinfo(transport: &dyn TransportTrait, hash_type: serde_json::Value, hash_or_height: serde_json::Value, use_index: serde_json::Value) -> Result<GettxoutsetinfoResponse, TransportError> {
    let params = vec![json!(hash_type), json!(hash_or_height), json!(use_index)];
    let raw = transport.send_request("gettxoutsetinfo", &params).await?;
    Ok(serde_json::from_value::<GettxoutsetinfoResponse>(raw)?)
}
