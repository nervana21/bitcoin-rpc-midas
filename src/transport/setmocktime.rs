//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Set the local time to given timestamp (-regtest only)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::setmocktime;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.setmocktime(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};

/// Calls the `setmocktime` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn setmocktime(
    transport: &dyn TransportTrait,
    timestamp: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(timestamp)];
    let raw = transport.send_request("setmocktime", &params).await?;
    Ok(raw)
}
