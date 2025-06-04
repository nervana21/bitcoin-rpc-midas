//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Rescan the local blockchain for wallet related transactions.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// The rescan is significantly faster when used on a descriptor wallet
/// and block filters are available (using startup option "-blockfilterindex=1").

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::rescanblockchain;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.rescanblockchain(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `rescanblockchain` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RescanblockchainResponse {
    /// The block height where the rescan started (the requested height or 0)
    pub start_height: u64,
    /// The height of the last rescanned block. May be null in rare cases if there was a reorg and the call didn't scan any blocks because they were already scanned in the background.
    pub stop_height: u64,
}



/// Calls the `rescanblockchain` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn rescanblockchain(transport: &dyn Transport, start_height: serde_json::Value, stop_height: serde_json::Value) -> Result<RescanblockchainResponse, TransportError> {
    let params = vec![json!(start_height), json!(stop_height)];
    let raw = transport.send_request("rescanblockchain", &params).await?;
    Ok(serde_json::from_value::<RescanblockchainResponse>(raw)?)
}
