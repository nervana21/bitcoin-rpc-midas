//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Analyzes and provides information about the current status of a PSBT and its inputs

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::analyzepsbt;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.analyzepsbt(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `analyzepsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AnalyzepsbtResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<serde_json::Value>>,
    /// Estimated vsize of the final signed transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_vsize: Option<u64>,
    /// Estimated feerate of the final signed transaction in BTC/kvB. Shown only if all UTXO slots in the PSBT have been filled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_feerate: Option<bitcoin::Amount>,
    /// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<bitcoin::Amount>,
    /// Role of the next person that this psbt needs to go to
    pub next: String,
    /// Error message (if there is one)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}



/// Calls the `analyzepsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn analyzepsbt(transport: &dyn Transport, psbt: serde_json::Value) -> Result<AnalyzepsbtResponse, TransportError> {
    let params = vec![json!(psbt)];
    let raw = transport.send_request("analyzepsbt", &params).await?;
    Ok(serde_json::from_value::<AnalyzepsbtResponse>(raw)?)
}
