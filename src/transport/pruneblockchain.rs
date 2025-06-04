//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest


/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::pruneblockchain;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.pruneblockchain(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `pruneblockchain` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PruneblockchainResponse {
    /// Height of the last block pruned
    pub field_0: u64,
}



/// Calls the `pruneblockchain` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn pruneblockchain(transport: &dyn Transport, height: serde_json::Value) -> Result<PruneblockchainResponse, TransportError> {
    let params = vec![json!(height)];
    let raw = transport.send_request("pruneblockchain", &params).await?;
    Ok(serde_json::from_value::<PruneblockchainResponse>(raw)?)
}
