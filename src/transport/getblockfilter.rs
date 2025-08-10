//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Retrieve a BIP 157 content filter for a particular block.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::getblockfilter;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getblockfilter(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Retrieve a BIP 157 content filter for a particular block.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetblockfilterResponse {
    pub filter: String,
    pub header: String,
}

/// Calls the `getblockfilter` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockfilter(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    filtertype: serde_json::Value,
) -> Result<GetblockfilterResponse, TransportError> {
    let params = vec![json!(blockhash), json!(filtertype)];
    let raw = transport.send_request("getblockfilter", &params).await?;
    Ok(serde_json::from_value::<GetblockfilterResponse>(raw)?)
}
