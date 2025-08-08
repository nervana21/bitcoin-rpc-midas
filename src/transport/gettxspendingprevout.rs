//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Scans the mempool to find transactions spending any of the given outputs

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::gettxspendingprevout;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.gettxspendingprevout(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Scans the mempool to find transactions spending any of the given outputs
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GettxspendingprevoutResponse(pub Vec<serde_json::Value>);

/// Calls the `gettxspendingprevout` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettxspendingprevout(
    transport: &dyn TransportTrait,
    outputs: serde_json::Value,
) -> Result<GettxspendingprevoutResponse, TransportError> {
    let params = vec![json!(outputs)];
    let raw = transport
        .send_request("gettxspendingprevout", &params)
        .await?;
    Ok(serde_json::from_value::<GettxspendingprevoutResponse>(raw)?)
}
