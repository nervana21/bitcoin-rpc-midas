//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
/// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getchaintips().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getchaintips;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getchaintips(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetchaintipsResponse(pub Vec<serde_json::Value>);

/// Calls the `getchaintips` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getchaintips(
    transport: &dyn TransportTrait,
) -> Result<GetchaintipsResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getchaintips", &params).await?;
    Ok(serde_json::from_value::<GetchaintipsResponse>(raw)?)
}
