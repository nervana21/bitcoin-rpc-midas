//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
/// Returns the total uptime of the server.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.uptime().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::uptime;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = uptime(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns the total uptime of the server.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UptimeResponse(pub u64);

/// Calls the `uptime` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn uptime(transport: &dyn TransportTrait) -> Result<UptimeResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("uptime", &params).await?;
    Ok(serde_json::from_value::<UptimeResponse>(raw)?)
}
