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
use transport::{Transport, TransportError};
/// Response for the `gettxoutsetinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettxoutsetinfoResponse {
    /// The block height (index) of the returned statistics
    pub height: u64,
    /// The hash of the block at which these statistics are calculated
    pub bestblock: String,
    /// The number of unspent transaction outputs
    pub txouts: u64,
    /// Database-independent, meaningless metric indicating the UTXO set size
    pub bogosize: u64,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_serialized_3: Option<String>,
    /// The serialized hash (only present if 'muhash' hash_type is chosen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muhash: Option<String>,
    /// The number of transactions with unspent outputs (not available when coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<u64>,
    /// The estimated size of the chainstate on disk (not available when coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<u64>,
    /// The total amount of coins in the UTXO set
    pub total_amount: bitcoin::Amount,
    /// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_unspendable_amount: Option<bitcoin::Amount>,
    /// Info on amounts in the block at this block height (only available if coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_info: Option<bitcoin::Block>,
}



/// Calls the `gettxoutsetinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettxoutsetinfo(transport: &dyn Transport, hash_type: serde_json::Value, hash_or_height: serde_json::Value, use_index: serde_json::Value) -> Result<GettxoutsetinfoResponse, TransportError> {
    let params = vec![json!(hash_type), json!(hash_or_height), json!(use_index)];
    let raw = transport.send_request("gettxoutsetinfo", &params).await?;
    Ok(serde_json::from_value::<GettxoutsetinfoResponse>(raw)?)
}
