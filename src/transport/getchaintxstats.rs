//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Compute statistics about the total number and rate of transactions in the chain.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getchaintxstats(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getchaintxstats;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getchaintxstats(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Compute statistics about the total number and rate of transactions in the chain.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetchaintxstatsResponse {
    pub time: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txcount: Option<u64>,
    pub window_final_block_hash: bitcoin::BlockHash,
    pub window_final_block_height: u64,
    pub window_block_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_interval: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_tx_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txrate: Option<f64>,
}

/// Calls the `getchaintxstats` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getchaintxstats(
    transport: &dyn TransportTrait,
    nblocks: serde_json::Value,
    blockhash: serde_json::Value,
) -> Result<GetchaintxstatsResponse, TransportError> {
    let params = vec![json!(nblocks), json!(blockhash)];
    let raw = transport.send_request("getchaintxstats", &params).await?;
    Ok(serde_json::from_value::<GetchaintxstatsResponse>(raw)?)
}
