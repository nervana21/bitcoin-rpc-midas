//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns the status of one or all available indices currently running in the node.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getindexinfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getindexinfo(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns the status of one or all available indices currently running in the node.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetindexinfoResponse(pub serde_json::Value);



/// Calls the `getindexinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getindexinfo(transport: &dyn TransportTrait, index_name: serde_json::Value) -> Result<GetindexinfoResponse, TransportError> {
    let params = vec![json!(index_name)];
    let raw = transport.send_request("getindexinfo", &params).await?;
    Ok(serde_json::from_value::<GetindexinfoResponse>(raw)?)
}
