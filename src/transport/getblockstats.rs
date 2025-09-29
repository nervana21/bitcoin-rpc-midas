//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won't work for some heights with pruning.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getblockstats(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getblockstats;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getblockstats(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
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
    pub ins: Option<u64>,
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
    pub outs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsidy: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtotal_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtotal_weight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtxs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_out: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_weight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalfee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase_actual: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc_actual: Option<u64>,
}

/// Calls the `getblockstats` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockstats(
    transport: &dyn TransportTrait,
    hash_or_height: serde_json::Value,
    stats: serde_json::Value,
) -> Result<GetblockstatsResponse, TransportError> {
    let params = vec![json!(hash_or_height), json!(stats)];
    let raw = transport.send_request("getblockstats", &params).await?;
    Ok(serde_json::from_value::<GetblockstatsResponse>(raw)?)
}
