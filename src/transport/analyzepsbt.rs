//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Analyzes and provides information about the current status of a PSBT and its inputs

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.analyzepsbt(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::analyzepsbt;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = analyzepsbt(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
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
