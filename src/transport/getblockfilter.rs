//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Retrieve a BIP 157 content filter for a particular block.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getblockfilter;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getblockfilter(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getblockfilter` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockfilterResponse {
    /// the hex-encoded filter data
    pub filter: String,
    /// the hex-encoded filter header
    pub header: String,
}



/// Calls the `getblockfilter` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockfilter(transport: &dyn Transport, blockhash: serde_json::Value, filtertype: serde_json::Value) -> Result<GetblockfilterResponse, TransportError> {
    let params = vec![json!(blockhash), json!(filtertype)];
    let raw = transport.send_request("getblockfilter", &params).await?;
    Ok(serde_json::from_value::<GetblockfilterResponse>(raw)?)
}
