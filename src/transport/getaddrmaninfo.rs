//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
/// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getaddrmaninfo().await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getaddrmaninfo;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getaddrmaninfo(&transport).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetaddrmaninfoResponse {
    pub network: serde_json::Value,
}

/// Calls the `getaddrmaninfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getaddrmaninfo(
    transport: &dyn TransportTrait,
) -> Result<GetaddrmaninfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getaddrmaninfo", &params).await?;
    Ok(serde_json::from_value::<GetaddrmaninfoResponse>(raw)?)
}
