//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::waitforblockheight;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.waitforblockheight(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `waitforblockheight` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WaitforblockheightResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}



/// Calls the `waitforblockheight` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn waitforblockheight(transport: &dyn Transport, height: serde_json::Value, timeout: serde_json::Value) -> Result<WaitforblockheightResponse, TransportError> {
    let params = vec![json!(height), json!(timeout)];
    let raw = transport.send_request("waitforblockheight", &params).await?;
    Ok(serde_json::from_value::<WaitforblockheightResponse>(raw)?)
}
