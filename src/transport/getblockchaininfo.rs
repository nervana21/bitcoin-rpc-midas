//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns an object containing various state info regarding blockchain processing.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getblockchaininfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getblockchaininfo().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getblockchaininfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockchaininfoResponse {
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// the height of the most-work fully-validated chain. The genesis block has height 0
    pub blocks: u64,
    /// the current number of headers we have validated
    pub headers: u64,
    /// the hash of the currently best block
    pub bestblockhash: String,
    /// the current difficulty
    pub difficulty: u64,
    /// The block time expressed in UNIX epoch time
    pub time: serde_json::Value,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: serde_json::Value,
    /// estimate of verification progress [0..1]
    pub verificationprogress: u64,
    /// (debug information) estimate of whether this node is in Initial Block Download mode
    pub initialblockdownload: bool,
    /// total amount of work in active chain, in hexadecimal
    pub chainwork: String,
    /// the estimated size of the block and undo files on disk
    pub size_on_disk: u64,
    /// if the blocks are subject to pruning
    pub pruned: bool,
    /// height of the last block pruned, plus one (only present if pruning is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pruneheight: Option<u64>,
    /// whether automatic pruning is enabled (only present if pruning is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_pruning: Option<bool>,
    /// the target size used by pruning (only present if automatic pruning is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune_target_size: Option<u64>,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}



/// Calls the `getblockchaininfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockchaininfo(transport: &dyn Transport) -> Result<GetblockchaininfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getblockchaininfo", &params).await?;
    Ok(serde_json::from_value::<GetblockchaininfoResponse>(raw)?)
}
