//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.gettxoutsetinfo(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::gettxoutsetinfo;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = gettxoutsetinfo(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
#[derive(Debug, Deserialize, Serialize)]
pub struct GettxoutsetinfoResponse {
    pub height: u64,
    pub bestblock: String,
    pub txouts: u64,
    pub bogosize: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_serialized_3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muhash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<u64>,
    pub total_amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_unspendable_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_info: Option<serde_json::Value>,
}

/// Calls the `gettxoutsetinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettxoutsetinfo(
    transport: &dyn TransportTrait,
    hash_type: serde_json::Value,
    hash_or_height: serde_json::Value,
    use_index: serde_json::Value,
) -> Result<GettxoutsetinfoResponse, TransportError> {
    let params = vec![json!(hash_type), json!(hash_or_height), json!(use_index)];
    let raw = transport.send_request("gettxoutsetinfo", &params).await?;
    Ok(serde_json::from_value::<GettxoutsetinfoResponse>(raw)?)
}
