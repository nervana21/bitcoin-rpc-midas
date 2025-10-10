//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns mempool data for given transaction
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getmempoolentry(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getmempoolentry;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getmempoolentry(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns mempool data for given transaction
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmempoolentryResponse {
    pub vsize: u64,
    pub weight: u64,
    pub time: serde_json::Value,
    pub height: u64,
    pub descendantcount: u64,
    pub descendantsize: u64,
    pub ancestorcount: u64,
    pub ancestorsize: u64,
    pub wtxid: bitcoin::Txid,
    pub fees: serde_json::Value,
    pub depends: Vec<serde_json::Value>,
    pub spentby: Vec<serde_json::Value>,
    pub bip125_replaceable: bool,
    pub unbroadcast: bool,
}

/// Calls the `getmempoolentry` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmempoolentry(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
) -> Result<GetmempoolentryResponse, TransportError> {
    let params = vec![json!(txid)];
    let raw = transport.send_request("getmempoolentry", &params).await?;
    Ok(serde_json::from_value::<GetmempoolentryResponse>(raw)?)
}
