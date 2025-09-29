//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.decodepsbt(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::decodepsbt;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = decodepsbt(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
#[derive(Debug, Deserialize, Serialize)]
pub struct DecodepsbtResponse {
    pub tx: serde_json::Value,
    pub global_xpubs: Vec<serde_json::Value>,
    pub psbt_version: u32,
    pub proprietary: Vec<serde_json::Value>,
    pub unknown: serde_json::Value,
    pub inputs: Vec<serde_json::Value>,
    pub outputs: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
}

/// Calls the `decodepsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn decodepsbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
) -> Result<DecodepsbtResponse, TransportError> {
    let params = vec![json!(psbt)];
    let raw = transport.send_request("decodepsbt", &params).await?;
    Ok(serde_json::from_value::<DecodepsbtResponse>(raw)?)
}
