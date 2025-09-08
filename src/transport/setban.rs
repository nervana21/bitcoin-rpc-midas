//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Attempts to add or remove an IP/Subnet from the banned list.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::setban;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.setban(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};

/// Calls the `setban` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn setban(
    transport: &dyn TransportTrait,
    subnet: serde_json::Value,
    command: serde_json::Value,
    bantime: serde_json::Value,
    absolute: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(subnet),
        json!(command),
        json!(bantime),
        json!(absolute),
    ];
    let raw = transport.send_request("setban", &params).await?;
    Ok(raw)
}
