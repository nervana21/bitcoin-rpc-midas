//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Returns the status of one or all available indices currently running in the node.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getindexinfo;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getindexinfo(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns the status of one or all available indices currently running in the node.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetindexinfoResponse {
    pub name: serde_json::Value,
}

/// Calls the `getindexinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getindexinfo(
    transport: &dyn TransportTrait,
    index_name: serde_json::Value,
) -> Result<GetindexinfoResponse, TransportError> {
    let params = vec![json!(index_name)];
    let raw = transport.send_request("getindexinfo", &params).await?;
    Ok(serde_json::from_value::<GetindexinfoResponse>(raw)?)
}
