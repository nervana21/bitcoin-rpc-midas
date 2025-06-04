//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getrawmempool;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getrawmempool(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getrawmempool` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrawmempoolResponse {
    pub field_0: Vec<serde_json::Value>,
    pub field_1: serde_json::Value,
    pub field_2: serde_json::Value,
}



/// Calls the `getrawmempool` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrawmempool(transport: &dyn Transport, verbose: serde_json::Value, mempool_sequence: serde_json::Value) -> Result<GetrawmempoolResponse, TransportError> {
    let params = vec![json!(verbose), json!(mempool_sequence)];
    let raw = transport.send_request("getrawmempool", &params).await?;
    Ok(serde_json::from_value::<GetrawmempoolResponse>(raw)?)
}
