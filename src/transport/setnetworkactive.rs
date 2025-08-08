//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Disable/enable all p2p network activity.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::setnetworkactive;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.setnetworkactive(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Disable/enable all p2p network activity.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SetnetworkactiveResponse(pub bool);

/// Calls the `setnetworkactive` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn setnetworkactive(
    transport: &dyn TransportTrait,
    state: serde_json::Value,
) -> Result<SetnetworkactiveResponse, TransportError> {
    let params = vec![json!(state)];
    let raw = transport.send_request("setnetworkactive", &params).await?;
    Ok(serde_json::from_value::<SetnetworkactiveResponse>(raw)?)
}
