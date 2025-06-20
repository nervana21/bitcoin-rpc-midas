//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::scanblocks;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.scanblocks(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
pub struct ScanblocksResponse {
        #[serde(skip_serializing_if = "Option::is_none")]
    pub from_height: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub to_height: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_blocks: Option<Vec<serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub current_height: Option<u64>,
}



/// Calls the `scanblocks` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn scanblocks(transport: &dyn TransportTrait, action: serde_json::Value, scanobjects: serde_json::Value, start_height: serde_json::Value, stop_height: serde_json::Value, filtertype: serde_json::Value, options: serde_json::Value) -> Result<ScanblocksResponse, TransportError> {
    let params = vec![json!(action), json!(scanobjects), json!(start_height), json!(stop_height), json!(filtertype), json!(options)];
    let raw = transport.send_request("scanblocks", &params).await?;
    Ok(serde_json::from_value::<ScanblocksResponse>(raw)?)
}
