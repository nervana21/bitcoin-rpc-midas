//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Get detailed information about in-wallet transaction <txid>

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::gettransaction;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.gettransaction(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Get detailed information about in-wallet transaction <txid>
#[derive(Debug, Deserialize, Serialize)]
pub struct GettransactionResponse {
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    pub confirmations: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<bitcoin::BlockHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockheight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockindex: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocktime: Option<serde_json::Value>,
    pub txid: bitcoin::Txid,
    pub wtxid: bitcoin::Txid,
    pub walletconflicts: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by_txid: Option<bitcoin::Txid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_txid: Option<bitcoin::Txid>,
    pub mempoolconflicts: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    pub time: serde_json::Value,
    pub timereceived: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub bip125_replaceable: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_descs: Option<Vec<serde_json::Value>>,
    pub details: Vec<serde_json::Value>,
    pub hex: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded: Option<serde_json::Value>,
    pub lastprocessedblock: serde_json::Value,
}



/// Calls the `gettransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettransaction(transport: &dyn TransportTrait, txid: serde_json::Value, include_watchonly: serde_json::Value, verbose: serde_json::Value) -> Result<GettransactionResponse, TransportError> {
    let params = vec![json!(txid), json!(include_watchonly), json!(verbose)];
    let raw = transport.send_request("gettransaction", &params).await?;
    Ok(serde_json::from_value::<GettransactionResponse>(raw)?)
}
