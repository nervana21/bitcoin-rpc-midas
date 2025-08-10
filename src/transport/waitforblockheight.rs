//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::waitforblockheight;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.waitforblockheight(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
pub struct WaitforblockheightResponse {
    pub hash: String,
    pub height: u64,
}

/// Calls the `waitforblockheight` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn waitforblockheight(
    transport: &dyn TransportTrait,
    height: serde_json::Value,
    timeout: serde_json::Value,
) -> Result<WaitforblockheightResponse, TransportError> {
    let params = vec![json!(height), json!(timeout)];
    let raw = transport
        .send_request("waitforblockheight", &params)
        .await?;
    Ok(serde_json::from_value::<WaitforblockheightResponse>(raw)?)
}
