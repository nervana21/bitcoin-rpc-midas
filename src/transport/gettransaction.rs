//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Get detailed information about in-wallet transaction <txid>

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::gettransaction;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.gettransaction(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `gettransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettransactionResponse {
    /// The amount in BTC
    pub amount: bitcoin::Amount,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<bitcoin::Amount>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: u64,
    /// Only present if the transaction's only input is a coinbase one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<bool>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
    /// The block hash containing the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<bitcoin::BlockHash>,
    /// The block height containing the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockheight: Option<u64>,
    /// The index of the transaction in the block that includes it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockindex: Option<u64>,
    /// The block time expressed in UNIX epoch time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocktime: Option<serde_json::Value>,
    /// The transaction id.
    pub txid: bitcoin::Txid,
    /// The hash of serialized transaction, including witness data.
    pub wtxid: bitcoin::Txid,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    pub walletconflicts: Vec<serde_json::Value>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by_txid: Option<bitcoin::Txid>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_txid: Option<bitcoin::Txid>,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    pub mempoolconflicts: Vec<serde_json::Value>,
    /// If a comment to is associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: serde_json::Value,
    /// The time received expressed in UNIX epoch time.
    pub timereceived: serde_json::Value,
    /// If a comment is associated with the transaction, only present if not empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// (\"yes|no|unknown\") Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    pub bip125_replaceable: String,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_descs: Option<Vec<serde_json::Value>>,
    pub details: Vec<serde_json::Value>,
    /// Raw data for transaction
    pub hex: String,
    /// The decoded transaction (only present when `verbose` is passed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded: Option<serde_json::Value>,
    /// hash and height of the block this information was generated on
    pub lastprocessedblock: bitcoin::Block,
}



/// Calls the `gettransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettransaction(transport: &dyn Transport, txid: serde_json::Value, include_watchonly: serde_json::Value, verbose: serde_json::Value) -> Result<GettransactionResponse, TransportError> {
    let params = vec![json!(txid), json!(include_watchonly), json!(verbose)];
    let raw = transport.send_request("gettransaction", &params).await?;
    Ok(serde_json::from_value::<GettransactionResponse>(raw)?)
}
