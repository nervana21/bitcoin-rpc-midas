//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
/// Returns an object with all balances in BTC.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getbalances().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getbalances;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getbalances(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns an object with all balances in BTC.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetbalancesResponse {
    pub mine: serde_json::Value,
    pub lastprocessedblock: serde_json::Value,
}

/// Calls the `getbalances` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getbalances(
    transport: &dyn TransportTrait,
) -> Result<GetbalancesResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getbalances", &params).await?;
    Ok(serde_json::from_value::<GetbalancesResponse>(raw)?)
}
