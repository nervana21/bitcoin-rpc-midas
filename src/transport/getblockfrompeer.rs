//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Attempt to fetch block from a given peer.
/// We must have the header for this block, e.g. using submitheader.
/// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
/// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
/// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
/// When a peer does not respond with a block, we will disconnect.
/// Note: The block could be re-pruned as soon as it is received.
/// Returns an empty JSON object if the request was successfully scheduled.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getblockfrompeer;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getblockfrompeer(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Attempt to fetch block from a given peer.
///
/// We must have the header for this block, e.g. using submitheader.
/// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
/// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
/// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
/// When a peer does not respond with a block, we will disconnect.
/// Note: The block could be re-pruned as soon as it is received.
///
/// Returns an empty JSON object if the request was successfully scheduled.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetblockfrompeerResponse(pub serde_json::Value);

/// Calls the `getblockfrompeer` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockfrompeer(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    peer_id: serde_json::Value,
) -> Result<GetblockfrompeerResponse, TransportError> {
    let params = vec![json!(blockhash), json!(peer_id)];
    let raw = transport.send_request("getblockfrompeer", &params).await?;
    Ok(serde_json::from_value::<GetblockfrompeerResponse>(raw)?)
}
