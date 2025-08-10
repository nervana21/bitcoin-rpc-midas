//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Returns an object containing various state info regarding deployments of consensus changes.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::getdeploymentinfo;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getdeploymentinfo(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns an object containing various state info regarding deployments of consensus changes.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetdeploymentinfoResponse {
    pub hash: String,
    pub height: u64,
    pub deployments: serde_json::Value,
}

/// Calls the `getdeploymentinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getdeploymentinfo(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
) -> Result<GetdeploymentinfoResponse, TransportError> {
    let params = vec![json!(blockhash)];
    let raw = transport.send_request("getdeploymentinfo", &params).await?;
    Ok(serde_json::from_value::<GetdeploymentinfoResponse>(raw)?)
}
