//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
/// Returns details on the active state of the TX memory pool.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getmempoolinfo().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getmempoolinfo;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getmempoolinfo(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns details on the active state of the TX memory pool.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmempoolinfoResponse {
    pub loaded: bool,
    pub size: u64,
    pub bytes: u64,
    pub usage: u64,
    pub total_fee: f64,
    pub maxmempool: u64,
    pub mempoolminfee: f64,
    pub minrelaytxfee: f64,
    pub incrementalrelayfee: f64,
    pub unbroadcastcount: u64,
    pub fullrbf: bool,
    pub permitbaremultisig: bool,
    pub maxdatacarriersize: u64,
}

/// Calls the `getmempoolinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmempoolinfo(
    transport: &dyn TransportTrait,
) -> Result<GetmempoolinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getmempoolinfo", &params).await?;
    Ok(serde_json::from_value::<GetmempoolinfoResponse>(raw)?)
}
