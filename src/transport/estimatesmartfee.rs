//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::estimatesmartfee;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.estimatesmartfee(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `estimatesmartfee` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EstimatesmartfeeResponse {
    /// estimate fee rate in BTC/kvB (only present if no errors were encountered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feerate: Option<u64>,
    /// Errors encountered during processing (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// block number where estimate was found
    /// The request target will be clamped between 2 and the highest target
    /// fee estimation is able to return based on how long it has been running.
    /// An error is returned if not enough transactions and blocks
    /// have been observed to make an estimate for any number of blocks.
    pub blocks: u64,
}



/// Calls the `estimatesmartfee` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn estimatesmartfee(transport: &dyn Transport, conf_target: serde_json::Value, estimate_mode: serde_json::Value) -> Result<EstimatesmartfeeResponse, TransportError> {
    let params = vec![json!(conf_target), json!(estimate_mode)];
    let raw = transport.send_request("estimatesmartfee", &params).await?;
    Ok(serde_json::from_value::<EstimatesmartfeeResponse>(raw)?)
}
