//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Permanently marks a block as invalid, as if it violated a consensus rule.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::invalidateblock;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.invalidateblock(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};

/// Calls the `invalidateblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn invalidateblock(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(blockhash)];
    let raw = transport.send_request("invalidateblock", &params).await?;
    Ok(raw)
}
