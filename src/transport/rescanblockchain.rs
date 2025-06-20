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
use transport::{TransportTrait, TransportError};
/// Rescan the local blockchain for wallet related transactions.
    /// Note: Use \"getwalletinfo\" to query the scanning progress.
    /// The rescan is significantly faster when used on a descriptor wallet
    /// and block filters are available (using startup option \"-blockfilterindex=1\").
#[derive(Debug, Deserialize, Serialize)]
pub struct RescanblockchainResponse {
    pub start_height: u64,
    pub stop_height: u64,
}



/// Calls the `rescanblockchain` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn rescanblockchain(transport: &dyn TransportTrait, start_height: serde_json::Value, stop_height: serde_json::Value) -> Result<RescanblockchainResponse, TransportError> {
    let params = vec![json!(start_height), json!(stop_height)];
    let raw = transport.send_request("rescanblockchain", &params).await?;
    Ok(serde_json::from_value::<RescanblockchainResponse>(raw)?)
}
