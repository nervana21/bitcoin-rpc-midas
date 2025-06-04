//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::savemempool;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.savemempool().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `savemempool` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SavemempoolResponse {
    /// the directory and file where the mempool was saved
    pub filename: String,
}



/// Calls the `savemempool` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn savemempool(transport: &dyn Transport) -> Result<SavemempoolResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("savemempool", &params).await?;
    Ok(serde_json::from_value::<SavemempoolResponse>(raw)?)
}
