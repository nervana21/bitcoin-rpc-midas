//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// WARNING: This interface is unstable and may disappear or change!
/// WARNING: This is an advanced API call that is tightly coupled to the specific
/// implementation of fee estimation. The parameters it can be called with
/// and the results it returns will change if the internal implementation changes.
/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible. Uses virtual transaction size as
/// defined in BIP 141 (witness data is discounted).
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.estimaterawfee(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::estimaterawfee;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = estimaterawfee(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// WARNING: This interface is unstable and may disappear or change!
///
/// WARNING: This is an advanced API call that is tightly coupled to the specific
/// implementation of fee estimation. The parameters it can be called with
/// and the results it returns will change if the internal implementation changes.
///
/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible. Uses virtual transaction size as
/// defined in BIP 141 (witness data is discounted).
#[derive(Debug, Deserialize, Serialize)]
pub struct EstimaterawfeeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long: Option<serde_json::Value>,
}

/// Calls the `estimaterawfee` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn estimaterawfee(
    transport: &dyn TransportTrait,
    conf_target: serde_json::Value,
    threshold: serde_json::Value,
) -> Result<EstimaterawfeeResponse, TransportError> {
    let params = vec![json!(conf_target), json!(threshold)];
    let raw = transport.send_request("estimaterawfee", &params).await?;
    Ok(serde_json::from_value::<EstimaterawfeeResponse>(raw)?)
}
