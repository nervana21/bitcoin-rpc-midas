//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns the number of connections to other nodes.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getconnectioncount;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getconnectioncount().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getconnectioncount` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetconnectioncountResponse {
    /// The connection count
    pub field_0: u64,
}



/// Calls the `getconnectioncount` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getconnectioncount(transport: &dyn Transport) -> Result<GetconnectioncountResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getconnectioncount", &params).await?;
    Ok(serde_json::from_value::<GetconnectioncountResponse>(raw)?)
}
