//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Disable/enable all p2p network activity.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::setnetworkactive;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.setnetworkactive(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `setnetworkactive` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SetnetworkactiveResponse {
    /// The value that was passed in
    pub field_0: bool,
}



/// Calls the `setnetworkactive` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn setnetworkactive(transport: &dyn Transport, state: serde_json::Value) -> Result<SetnetworkactiveResponse, TransportError> {
    let params = vec![json!(state)];
    let raw = transport.send_request("setnetworkactive", &params).await?;
    Ok(serde_json::from_value::<SetnetworkactiveResponse>(raw)?)
}
