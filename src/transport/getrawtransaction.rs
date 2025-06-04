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
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getrawtransaction(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getrawtransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrawtransactionResponse {
    /// The serialized transaction as a hex-encoded string for 'txid'
    pub data: String,
    pub field_1: serde_json::Value,
    pub field_2: serde_json::Value,
}



/// Calls the `getrawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrawtransaction(transport: &dyn Transport, txid: serde_json::Value, verbosity: serde_json::Value, blockhash: serde_json::Value) -> Result<GetrawtransactionResponse, TransportError> {
    let params = vec![json!(txid), json!(verbosity), json!(blockhash)];
    let raw = transport.send_request("getrawtransaction", &params).await?;
    Ok(serde_json::from_value::<GetrawtransactionResponse>(raw)?)
}
