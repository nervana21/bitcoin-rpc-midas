//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
/// List all manually banned IPs/Subnets.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.listbanned().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::listbanned;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = listbanned(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// List all manually banned IPs/Subnets.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListbannedResponse(pub Vec<serde_json::Value>);

/// Calls the `listbanned` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listbanned(
    transport: &dyn TransportTrait,
) -> Result<ListbannedResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listbanned", &params).await?;
    Ok(serde_json::from_value::<ListbannedResponse>(raw)?)
}
