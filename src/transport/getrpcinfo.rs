//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
/// Returns details of the RPC server.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getrpcinfo().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getrpcinfo;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getrpcinfo(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns details of the RPC server.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetrpcinfoResponse {
    pub active_commands: Vec<serde_json::Value>,
    pub logpath: String,
}

/// Calls the `getrpcinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrpcinfo(
    transport: &dyn TransportTrait,
) -> Result<GetrpcinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getrpcinfo", &params).await?;
    Ok(serde_json::from_value::<GetrpcinfoResponse>(raw)?)
}
