//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Scans the mempool to find transactions spending any of the given outputs

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::gettxspendingprevout;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.gettxspendingprevout(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `gettxspendingprevout` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettxspendingprevoutResponse {
    pub field_0: Vec<serde_json::Value>,
}



/// Calls the `gettxspendingprevout` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettxspendingprevout(transport: &dyn Transport, outputs: serde_json::Value) -> Result<GettxspendingprevoutResponse, TransportError> {
    let params = vec![json!(outputs)];
    let raw = transport.send_request("gettxspendingprevout", &params).await?;
    Ok(serde_json::from_value::<GettxspendingprevoutResponse>(raw)?)
}
