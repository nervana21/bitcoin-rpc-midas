//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Waits for any new block and returns useful info about it.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::waitfornewblock;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.waitfornewblock(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Waits for any new block and returns useful info about it.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
pub struct WaitfornewblockResponse {
    pub hash: String,
    pub height: u64,
}

/// Calls the `waitfornewblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn waitfornewblock(
    transport: &dyn TransportTrait,
    timeout: serde_json::Value,
    current_tip: serde_json::Value,
) -> Result<WaitfornewblockResponse, TransportError> {
    let params = vec![json!(timeout), json!(current_tip)];
    let raw = transport.send_request("waitfornewblock", &params).await?;
    Ok(serde_json::from_value::<WaitfornewblockResponse>(raw)?)
}
