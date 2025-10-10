//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
/// Returns an object containing various state info regarding P2P networking.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getnetworkinfo().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getnetworkinfo;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getnetworkinfo(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns an object containing various state info regarding P2P networking.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetnetworkinfoResponse {
    pub version: u32,
    pub subversion: String,
    pub protocolversion: u32,
    pub localservices: String,
    pub localservicesnames: Vec<serde_json::Value>,
    pub localrelay: bool,
    pub timeoffset: u64,
    pub connections: u64,
    pub connections_in: u64,
    pub connections_out: u64,
    pub networkactive: bool,
    pub networks: Vec<serde_json::Value>,
    pub relayfee: f64,
    pub incrementalfee: f64,
    pub localaddresses: Vec<String>,
    pub warnings: Vec<serde_json::Value>,
}

/// Calls the `getnetworkinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getnetworkinfo(
    transport: &dyn TransportTrait,
) -> Result<GetnetworkinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getnetworkinfo", &params).await?;
    Ok(serde_json::from_value::<GetnetworkinfoResponse>(raw)?)
}
