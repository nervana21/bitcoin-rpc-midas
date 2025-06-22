//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Compute statistics about the total number and rate of transactions in the chain.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getchaintxstats;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getchaintxstats(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Compute statistics about the total number and rate of transactions in the chain.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetchaintxstatsResponse {
    pub time: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txcount: Option<u64>,
    pub window_final_block_hash: bitcoin::BlockHash,
    pub window_final_block_height: u64,
    pub window_block_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_interval: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_tx_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txrate: Option<f64>,
}



/// Calls the `getchaintxstats` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getchaintxstats(transport: &dyn TransportTrait, nblocks: serde_json::Value, blockhash: serde_json::Value) -> Result<GetchaintxstatsResponse, TransportError> {
    let params = vec![json!(nblocks), json!(blockhash)];
    let raw = transport.send_request("getchaintxstats", &params).await?;
    Ok(serde_json::from_value::<GetchaintxstatsResponse>(raw)?)
}
