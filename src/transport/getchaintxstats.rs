//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Compute statistics about the total number and rate of transactions in the chain.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getchaintxstats;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getchaintxstats(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getchaintxstats` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetchaintxstatsResponse {
    /// The timestamp for the final block in the window, expressed in UNIX epoch time
    pub time: serde_json::Value,
    /// The total number of transactions in the chain up to that point, if known. It may be unknown when using assumeutxo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txcount: Option<u64>,
    /// The hash of the final block in the window
    pub window_final_block_hash: String,
    /// The height of the final block in the window.
    pub window_final_block_height: u64,
    /// Size of the window in number of blocks
    pub window_block_count: u64,
    /// The elapsed time in the window in seconds. Only returned if \"window_block_count\" is > 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_interval: Option<u64>,
    /// The number of transactions in the window. Only returned if \"window_block_count\" is > 0 and if txcount exists for the start and end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_tx_count: Option<u64>,
    /// The average rate of transactions per second in the window. Only returned if \"window_interval\" is > 0 and if window_tx_count exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txrate: Option<u64>,
}



/// Calls the `getchaintxstats` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getchaintxstats(transport: &dyn Transport, nblocks: serde_json::Value, blockhash: serde_json::Value) -> Result<GetchaintxstatsResponse, TransportError> {
    let params = vec![json!(nblocks), json!(blockhash)];
    let raw = transport.send_request("getchaintxstats", &params).await?;
    Ok(serde_json::from_value::<GetchaintxstatsResponse>(raw)?)
}
