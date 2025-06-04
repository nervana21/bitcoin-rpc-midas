//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Waits for a specific new block and returns useful info about it.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::waitforblock;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.waitforblock(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `waitforblock` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WaitforblockResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}



/// Calls the `waitforblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn waitforblock(transport: &dyn Transport, blockhash: serde_json::Value, timeout: serde_json::Value) -> Result<WaitforblockResponse, TransportError> {
    let params = vec![json!(blockhash), json!(timeout)];
    let raw = transport.send_request("waitforblock", &params).await?;
    Ok(serde_json::from_value::<WaitforblockResponse>(raw)?)
}
