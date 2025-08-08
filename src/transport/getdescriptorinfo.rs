//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Analyses a descriptor.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getdescriptorinfo;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getdescriptorinfo(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Analyses a descriptor.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetdescriptorinfoResponse {
    pub descriptor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipath_expansion: Option<Vec<serde_json::Value>>,
    pub checksum: String,
    pub isrange: bool,
    pub issolvable: bool,
    pub hasprivatekeys: bool,
}

/// Calls the `getdescriptorinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getdescriptorinfo(
    transport: &dyn TransportTrait,
    descriptor: serde_json::Value,
) -> Result<GetdescriptorinfoResponse, TransportError> {
    let params = vec![json!(descriptor)];
    let raw = transport.send_request("getdescriptorinfo", &params).await?;
    Ok(serde_json::from_value::<GetdescriptorinfoResponse>(raw)?)
}
