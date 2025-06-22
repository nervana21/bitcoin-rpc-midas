//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns the height of the most-work fully-validated chain.
/// The genesis block has height 0.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getblockcount;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getblockcount().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns the height of the most-work fully-validated chain.
    /// The genesis block has height 0.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetblockcountResponse(pub u64);



/// Calls the `getblockcount` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockcount(transport: &dyn TransportTrait) -> Result<GetblockcountResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getblockcount", &params).await?;
    Ok(serde_json::from_value::<GetblockcountResponse>(raw)?)
}
