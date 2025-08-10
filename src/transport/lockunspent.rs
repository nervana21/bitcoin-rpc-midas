//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Updates list of temporarily unspendable outputs.
/// Temporarily lock (unlock=false) or unlock (unlock=true) specified transaction outputs.
/// If no transaction outputs are specified when unlocking then all current locked transaction outputs are unlocked.
/// A locked transaction output will not be chosen by automatic coin selection, when spending bitcoins.
/// Manually selected coins are automatically unlocked.
/// Locks are stored in memory only, unless persistent=true, in which case they will be written to the
/// wallet database and loaded on node start. Unwritten (persistent=false) locks are always cleared
/// (by virtue of process exit) when a node stops or fails. Unlocking will clear both persistent and not.
/// Also see the listunspent call

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::lockunspent;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.lockunspent(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Updates list of temporarily unspendable outputs.
/// Temporarily lock (unlock=false) or unlock (unlock=true) specified transaction outputs.
/// If no transaction outputs are specified when unlocking then all current locked transaction outputs are unlocked.
/// A locked transaction output will not be chosen by automatic coin selection, when spending bitcoins.
/// Manually selected coins are automatically unlocked.
/// Locks are stored in memory only, unless persistent=true, in which case they will be written to the
/// wallet database and loaded on node start. Unwritten (persistent=false) locks are always cleared
/// (by virtue of process exit) when a node stops or fails. Unlocking will clear both persistent and not.
/// Also see the listunspent call
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LockunspentResponse(pub bool);

/// Calls the `lockunspent` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn lockunspent(
    transport: &dyn TransportTrait,
    unlock: serde_json::Value,
    transactions: serde_json::Value,
    persistent: serde_json::Value,
) -> Result<LockunspentResponse, TransportError> {
    let params = vec![json!(unlock), json!(transactions), json!(persistent)];
    let raw = transport.send_request("lockunspent", &params).await?;
    Ok(serde_json::from_value::<LockunspentResponse>(raw)?)
}
