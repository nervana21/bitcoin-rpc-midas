//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getblockheader;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getblockheader(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetblockheaderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkleroot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediantime: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bits: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
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
}

/// Calls the `getblockheader` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockheader(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<GetblockheaderResponse, TransportError> {
    let params = vec![json!(blockhash), json!(verbose)];
    let raw = transport.send_request("getblockheader", &params).await?;
    Ok(serde_json::from_value::<GetblockheaderResponse>(raw)?)
}
