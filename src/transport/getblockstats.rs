//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won't work for some heights with pruning.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getblockstats;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getblockstats(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Compute per block statistics for a given window. All amounts are in satoshis.
    /// It won't work for some heights with pruning.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetblockstatsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgfee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgfeerate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgtxsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<bitcoin::BlockHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feerate_percentiles: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfeerate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxtxsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medianfee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediantime: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediantxsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minfee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minfeerate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mintxsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsidy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtotal_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtotal_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtxs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_out: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalfee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase_actual: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc_actual: Option<u64>,
}



/// Calls the `getblockstats` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockstats(transport: &dyn TransportTrait, hash_or_height: serde_json::Value, stats: serde_json::Value) -> Result<GetblockstatsResponse, TransportError> {
    let params = vec![json!(hash_or_height), json!(stats)];
    let raw = transport.send_request("getblockstats", &params).await?;
    Ok(serde_json::from_value::<GetblockstatsResponse>(raw)?)
}
