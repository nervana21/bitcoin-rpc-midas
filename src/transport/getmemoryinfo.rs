//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Returns an object containing information about memory usage.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getmemoryinfo;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getmemoryinfo(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns an object containing information about memory usage.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmemoryinfoResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<serde_json::Value>,
}

/// Calls the `getmemoryinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmemoryinfo(
    transport: &dyn TransportTrait,
    mode: serde_json::Value,
) -> Result<GetmemoryinfoResponse, TransportError> {
    let params = vec![json!(mode)];
    let raw = transport.send_request("getmemoryinfo", &params).await?;
    Ok(serde_json::from_value::<GetmemoryinfoResponse>(raw)?)
}
