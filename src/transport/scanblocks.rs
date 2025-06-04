//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::scanblocks;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.scanblocks(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `scanblocks` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanblocksResponse {
    pub field_1: serde_json::Value,
    pub field_2: serde_json::Value,
    /// True if scan will be aborted (not necessarily before this RPC returns), or false if there is no scan to abort
    pub success: bool,
}



/// Calls the `scanblocks` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn scanblocks(transport: &dyn Transport, action: serde_json::Value, scanobjects: serde_json::Value, start_height: serde_json::Value, stop_height: serde_json::Value, filtertype: serde_json::Value, options: serde_json::Value) -> Result<ScanblocksResponse, TransportError> {
    let params = vec![json!(action), json!(scanobjects), json!(start_height), json!(stop_height), json!(filtertype), json!(options)];
    let raw = transport.send_request("scanblocks", &params).await?;
    Ok(serde_json::from_value::<ScanblocksResponse>(raw)?)
}
