//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Returns information on all address manager entries for the new and tried tables.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getrawaddrman().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getrawaddrman;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getrawaddrman(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Returns information on all address manager entries for the new and tried tables.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetrawaddrmanResponse {
    pub table: serde_json::Value,
}

/// Calls the `getrawaddrman` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrawaddrman(
    transport: &dyn TransportTrait,
) -> Result<GetrawaddrmanResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getrawaddrman", &params).await?;
    Ok(serde_json::from_value::<GetrawaddrmanResponse>(raw)?)
}
