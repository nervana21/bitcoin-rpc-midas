//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns the hash of the best (tip) block in the most-work fully-validated chain.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getbestblockhash;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getbestblockhash().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getbestblockhash` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbestblockhashResponse {
    /// the block hash, hex-encoded
    pub field_0: String,
}



/// Calls the `getbestblockhash` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getbestblockhash(transport: &dyn Transport) -> Result<GetbestblockhashResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getbestblockhash", &params).await?;
    Ok(serde_json::from_value::<GetbestblockhashResponse>(raw)?)
}
