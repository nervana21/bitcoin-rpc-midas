//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Waits for a specific new block and returns useful info about it.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::waitforblock;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.waitforblock(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Waits for a specific new block and returns useful info about it.
    /// 
    /// Returns the current block on timeout or exit.
    /// 
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
pub struct WaitforblockResponse {
    pub hash: String,
    pub height: u64,
}



/// Calls the `waitforblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn waitforblock(transport: &dyn TransportTrait, blockhash: serde_json::Value, timeout: serde_json::Value) -> Result<WaitforblockResponse, TransportError> {
    let params = vec![json!(blockhash), json!(timeout)];
    let raw = transport.send_request("waitforblock", &params).await?;
    Ok(serde_json::from_value::<WaitforblockResponse>(raw)?)
}
