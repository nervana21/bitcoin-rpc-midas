//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
/// Requires `-prune` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via `getblockfrompeer`), local deletion is irreversible.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::pruneblockchain;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.pruneblockchain(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
    /// Requires `-prune` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via `getblockfrompeer`), local deletion is irreversible.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PruneblockchainResponse(pub u64);



/// Calls the `pruneblockchain` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn pruneblockchain(transport: &dyn TransportTrait, height: serde_json::Value) -> Result<PruneblockchainResponse, TransportError> {
    let params = vec![json!(height)];
    let raw = transport.send_request("pruneblockchain", &params).await?;
    Ok(serde_json::from_value::<PruneblockchainResponse>(raw)?)
}
