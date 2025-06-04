//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns a json object containing mining-related information.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getmininginfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getmininginfo().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getmininginfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmininginfoResponse {
    /// The current block
    pub blocks: u64,
    /// The block weight of the last assembled block (only present if a block was ever assembled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentblockweight: Option<u64>,
    /// The number of block transactions of the last assembled block (only present if a block was ever assembled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentblocktx: Option<u64>,
    /// The current difficulty
    pub difficulty: u64,
    /// The network hashes per second
    pub networkhashps: u64,
    /// The size of the mempool
    pub pooledtx: u64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}



/// Calls the `getmininginfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmininginfo(transport: &dyn Transport) -> Result<GetmininginfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getmininginfo", &params).await?;
    Ok(serde_json::from_value::<GetmininginfoResponse>(raw)?)
}
