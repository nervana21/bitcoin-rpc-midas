//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
/// Returns the height of the most-work fully-validated chain.
/// The genesis block has height 0.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getblockcount().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getblockcount;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getblockcount(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns the height of the most-work fully-validated chain.
/// The genesis block has height 0.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetblockcountResponse(pub u64);

/// Calls the `getblockcount` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockcount(
    transport: &dyn TransportTrait,
) -> Result<GetblockcountResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getblockcount", &params).await?;
    Ok(serde_json::from_value::<GetblockcountResponse>(raw)?)
}
