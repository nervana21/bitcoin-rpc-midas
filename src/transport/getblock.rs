//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getblock;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getblock(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getblock` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockResponse {
    /// A string that is serialized, hex-encoded data for block 'hash'
    pub field_0: String,
    pub field_1: serde_json::Value,
    pub field_2: serde_json::Value,
    pub field_3: serde_json::Value,
}



/// Calls the `getblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblock(transport: &dyn Transport, blockhash: serde_json::Value, verbosity: serde_json::Value) -> Result<GetblockResponse, TransportError> {
    let params = vec![json!(blockhash), json!(verbosity)];
    let raw = transport.send_request("getblock", &params).await?;
    Ok(serde_json::from_value::<GetblockResponse>(raw)?)
}
