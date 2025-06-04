//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns an object containing various state info regarding deployments of consensus changes.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getdeploymentinfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getdeploymentinfo(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getdeploymentinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetdeploymentinfoResponse {
    /// requested block hash (or tip)
    pub hash: String,
    /// requested block height (or tip)
    pub height: u64,
    pub deployments: serde_json::Value,
}



/// Calls the `getdeploymentinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getdeploymentinfo(transport: &dyn Transport, blockhash: serde_json::Value) -> Result<GetdeploymentinfoResponse, TransportError> {
    let params = vec![json!(blockhash)];
    let raw = transport.send_request("getdeploymentinfo", &params).await?;
    Ok(serde_json::from_value::<GetdeploymentinfoResponse>(raw)?)
}
