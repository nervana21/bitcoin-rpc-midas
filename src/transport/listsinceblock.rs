//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
/// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::listsinceblock;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.listsinceblock(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `listsinceblock` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListsinceblockResponse {
    pub transactions: Vec<serde_json::Value>,
    /// <structure is the same as \"transactions\" above, only present if include_removed=true>
    /// Note: transactions that were re-added in the active chain will appear as-is in this array, and may thus have a positive confirmation count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<serde_json::Value>>,
    /// The hash of the block (target_confirmations-1) from the best block on the main chain, or the genesis hash if the referenced block does not exist yet. This is typically used to feed back into listsinceblock the next time you call it. So you would generally use a target_confirmations of say 6, so you will be continually re-notified of transactions until they've reached 6 confirmations plus any new ones
    pub lastblock: String,
}



/// Calls the `listsinceblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listsinceblock(transport: &dyn Transport, blockhash: serde_json::Value, target_confirmations: serde_json::Value, include_watchonly: serde_json::Value, include_removed: serde_json::Value, include_change: serde_json::Value, label: serde_json::Value) -> Result<ListsinceblockResponse, TransportError> {
    let params = vec![json!(blockhash), json!(target_confirmations), json!(include_watchonly), json!(include_removed), json!(include_change), json!(label)];
    let raw = transport.send_request("listsinceblock", &params).await?;
    Ok(serde_json::from_value::<ListsinceblockResponse>(raw)?)
}
