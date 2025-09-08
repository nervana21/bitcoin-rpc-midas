//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getaddednodeinfo;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getaddednodeinfo(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetaddednodeinfoResponse(pub Vec<serde_json::Value>);

/// Calls the `getaddednodeinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getaddednodeinfo(
    transport: &dyn TransportTrait,
    node: serde_json::Value,
) -> Result<GetaddednodeinfoResponse, TransportError> {
    let params = vec![json!(node)];
    let raw = transport.send_request("getaddednodeinfo", &params).await?;
    Ok(serde_json::from_value::<GetaddednodeinfoResponse>(raw)?)
}
