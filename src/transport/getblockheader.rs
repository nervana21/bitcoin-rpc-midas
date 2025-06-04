//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getblockheader;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getblockheader(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getblockheader` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockheaderResponse {
    pub field_0: serde_json::Value,
    /// A string that is serialized, hex-encoded data for block 'hash'
    pub field_1: String,
}



/// Calls the `getblockheader` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockheader(transport: &dyn Transport, blockhash: serde_json::Value, verbose: serde_json::Value) -> Result<GetblockheaderResponse, TransportError> {
    let params = vec![json!(blockhash), json!(verbose)];
    let raw = transport.send_request("getblockheader", &params).await?;
    Ok(serde_json::from_value::<GetblockheaderResponse>(raw)?)
}
