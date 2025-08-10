//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Mark in-wallet transaction <txid> as abandoned
/// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
/// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
/// It only works on transactions which are not included in a block and are not currently in the mempool.
/// It has no effect on transactions which are already abandoned.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::abandontransaction;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.abandontransaction(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};

/// Calls the `abandontransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn abandontransaction(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid)];
    let raw = transport
        .send_request("abandontransaction", &params)
        .await?;
    Ok(raw)
}
