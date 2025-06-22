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
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getblock(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
    /// If verbosity is 1, returns an Object with information about block <hash>.
    /// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
    /// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Debug, Deserialize, Serialize)]
pub struct GetblockResponse {
        #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub strippedsize: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub version_hex: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub merkleroot: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<Vec<serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub mediantime: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub bits: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub chainwork: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub n_tx: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub previousblockhash: Option<bitcoin::BlockHash>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub nextblockhash: Option<bitcoin::BlockHash>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub field_0: Option<serde_json::Value>,
}



/// Calls the `getblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblock(transport: &dyn TransportTrait, blockhash: serde_json::Value, verbosity: serde_json::Value) -> Result<GetblockResponse, TransportError> {
    let params = vec![json!(blockhash), json!(verbosity)];
    let raw = transport.send_request("getblock", &params).await?;
    Ok(serde_json::from_value::<GetblockResponse>(raw)?)
}
