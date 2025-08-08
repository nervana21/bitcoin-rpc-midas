//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// Hint: Use gettransaction for wallet transactions.
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getrawtransaction;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getrawtransaction(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
///
/// Hint: Use gettransaction for wallet transactions.
///
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetrawtransactionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_active_chain: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<bitcoin::BlockHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocktime: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locktime: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vin: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vout: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_0: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
}

/// Calls the `getrawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrawtransaction(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    verbosity: serde_json::Value,
    blockhash: serde_json::Value,
) -> Result<GetrawtransactionResponse, TransportError> {
    let params = vec![json!(txid), json!(verbosity), json!(blockhash)];
    let raw = transport.send_request("getrawtransaction", &params).await?;
    Ok(serde_json::from_value::<GetrawtransactionResponse>(raw)?)
}
