//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::removeprunedfunds;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.removeprunedfunds(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};

/// Calls the `removeprunedfunds` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn removeprunedfunds(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid)];
    let raw = transport.send_request("removeprunedfunds", &params).await?;
    Ok(raw)
}
