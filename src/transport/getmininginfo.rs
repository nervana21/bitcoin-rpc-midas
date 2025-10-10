//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
/// Returns a json object containing mining-related information.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getmininginfo().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getmininginfo;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getmininginfo(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns a json object containing mining-related information.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmininginfoResponse {
    pub blocks: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentblockweight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentblocktx: Option<u64>,
    pub bits: String,
    pub difficulty: f64,
    pub target: String,
    pub networkhashps: u64,
    pub pooledtx: u64,
    pub blockmintxfee: f64,
    pub chain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signet_challenge: Option<String>,
    pub next: serde_json::Value,
    pub warnings: Vec<serde_json::Value>,
}

/// Calls the `getmininginfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmininginfo(
    transport: &dyn TransportTrait,
) -> Result<GetmininginfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getmininginfo", &params).await?;
    Ok(serde_json::from_value::<GetmininginfoResponse>(raw)?)
}
