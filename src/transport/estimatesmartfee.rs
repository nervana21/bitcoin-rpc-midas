//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::estimatesmartfee;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.estimatesmartfee(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
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
