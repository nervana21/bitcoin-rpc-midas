//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Analyzes and provides information about the current status of a PSBT and its inputs

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::analyzepsbt;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.analyzepsbt(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Analyzes and provides information about the current status of a PSBT and its inputs
#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyzepsbtResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_vsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_feerate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    pub next: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Calls the `analyzepsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn analyzepsbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
) -> Result<AnalyzepsbtResponse, TransportError> {
    let params = vec![json!(psbt)];
    let raw = transport.send_request("analyzepsbt", &params).await?;
    Ok(serde_json::from_value::<AnalyzepsbtResponse>(raw)?)
}
