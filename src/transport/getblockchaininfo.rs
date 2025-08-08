//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns an object containing various state info regarding blockchain processing.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getblockchaininfo;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getblockchaininfo().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns an object containing various state info regarding blockchain processing.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetblockchaininfoResponse {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub bestblockhash: bitcoin::BlockHash,
    pub bits: String,
    pub target: String,
    pub difficulty: f64,
    pub time: serde_json::Value,
    pub mediantime: serde_json::Value,
    pub verificationprogress: f64,
    pub initialblockdownload: bool,
    pub chainwork: String,
    pub size_on_disk: u64,
    pub pruned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pruneheight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_pruning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune_target_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signet_challenge: Option<String>,
    pub warnings: Vec<serde_json::Value>,
}



/// Calls the `getblockchaininfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockchaininfo(transport: &dyn TransportTrait) -> Result<GetblockchaininfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getblockchaininfo", &params).await?;
    Ok(serde_json::from_value::<GetblockchaininfoResponse>(raw)?)
}
