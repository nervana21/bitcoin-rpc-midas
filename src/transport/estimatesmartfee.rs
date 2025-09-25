//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.estimatesmartfee(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::estimatesmartfee;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = estimatesmartfee(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).
#[derive(Debug, Deserialize, Serialize)]
pub struct EstimatesmartfeeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feerate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<serde_json::Value>>,
    pub blocks: u64,
}

/// Calls the `estimatesmartfee` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn estimatesmartfee(
    transport: &dyn TransportTrait,
    conf_target: serde_json::Value,
    estimate_mode: serde_json::Value,
) -> Result<EstimatesmartfeeResponse, TransportError> {
    let params = vec![json!(conf_target), json!(estimate_mode)];
    let raw = transport.send_request("estimatesmartfee", &params).await?;
    Ok(serde_json::from_value::<EstimatesmartfeeResponse>(raw)?)
}
