//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Bump the scheduler into the future (-regtest only)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::mockscheduler;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.mockscheduler(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};

/// Calls the `mockscheduler` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn mockscheduler(
    transport: &dyn TransportTrait,
    delta_time: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(delta_time)];
    let raw = transport.send_request("mockscheduler", &params).await?;
    Ok(raw)
}
