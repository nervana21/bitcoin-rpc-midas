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
use transport::{Transport, TransportError};
/// Response for the `getblockstats` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockstatsResponse {
    /// Average fee in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgfee: Option<u64>,
    /// Average feerate (in satoshis per virtual byte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgfeerate: Option<u64>,
    /// Average transaction size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgtxsize: Option<u64>,
    /// The block hash (to check for potential reorgs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<bitcoin::BlockHash>,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feerate_percentiles: Option<serde_json::Value>,
    /// The height of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    /// The number of inputs (excluding coinbase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins: Option<u64>,
    /// Maximum fee in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfee: Option<u64>,
    /// Maximum feerate (in satoshis per virtual byte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfeerate: Option<u64>,
    /// Maximum transaction size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxtxsize: Option<u64>,
    /// Truncated median fee in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medianfee: Option<u64>,
    /// The block median time past
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediantime: Option<u64>,
    /// Truncated median transaction size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediantxsize: Option<u64>,
    /// Minimum fee in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minfee: Option<u64>,
    /// Minimum feerate (in satoshis per virtual byte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minfeerate: Option<u64>,
    /// Minimum transaction size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mintxsize: Option<u64>,
    /// The number of outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outs: Option<u64>,
    /// The block subsidy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsidy: Option<u64>,
    /// Total size of all segwit transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtotal_size: Option<u64>,
    /// Total weight of all segwit transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtotal_weight: Option<u64>,
    /// The number of segwit transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtxs: Option<u64>,
    /// The block time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    /// Total amount in all outputs (excluding coinbase and thus reward [ie subsidy + totalfee])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_out: Option<u64>,
    /// Total size of all non-coinbase transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
    /// Total weight of all non-coinbase transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_weight: Option<u64>,
    /// The fee total
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalfee: Option<u64>,
    /// The number of transactions (including coinbase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs: Option<u64>,
    /// The increase/decrease in the number of unspent outputs (not discounting op_return and similar)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase: Option<u64>,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc: Option<u64>,
    /// The increase/decrease in the number of unspent outputs, not counting unspendables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase_actual: Option<u64>,
    /// The increase/decrease in size for the utxo index, not counting unspendables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc_actual: Option<u64>,
}



/// Calls the `getblockstats` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockstats(transport: &dyn Transport, hash_or_height: serde_json::Value, stats: serde_json::Value) -> Result<GetblockstatsResponse, TransportError> {
    let params = vec![json!(hash_or_height), json!(stats)];
    let raw = transport.send_request("getblockstats", &params).await?;
    Ok(serde_json::from_value::<GetblockstatsResponse>(raw)?)
}
