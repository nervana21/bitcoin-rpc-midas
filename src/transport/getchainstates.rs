//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Return information about chainstates.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getchainstates;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getchainstates().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getchainstates` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetchainstatesResponse {
    /// the number of headers seen so far
    pub headers: u64,
    /// list of the chainstates ordered by work, with the most-work (active) chainstate last
    pub chainstates: Vec<serde_json::Value>,
}



/// Calls the `getchainstates` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getchainstates(transport: &dyn Transport) -> Result<GetchainstatesResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getchainstates", &params).await?;
    Ok(serde_json::from_value::<GetchainstatesResponse>(raw)?)
}
