//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// WARNING: This interface is unstable and may disappear or change!
/// WARNING: This is an advanced API call that is tightly coupled to the specific
/// implementation of fee estimation. The parameters it can be called with
/// and the results it returns will change if the internal implementation changes.
/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible. Uses virtual transaction size as
/// defined in BIP 141 (witness data is discounted).

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::estimaterawfee;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.estimaterawfee(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
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
