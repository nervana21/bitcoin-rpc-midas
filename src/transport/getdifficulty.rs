//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getdifficulty().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getdifficulty;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getdifficulty(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetdifficultyResponse(pub f64);

/// Calls the `getdifficulty` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getdifficulty(
    transport: &dyn TransportTrait,
) -> Result<GetdifficultyResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getdifficulty", &params).await?;
    Ok(serde_json::from_value::<GetdifficultyResponse>(raw)?)
}
