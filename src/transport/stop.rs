//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Request a graceful shutdown of Bitcoin Core.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::stop;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.stop(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Request a graceful shutdown of Bitcoin Core.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StopResponse(pub String);

/// Calls the `stop` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn stop(
    transport: &dyn TransportTrait,
    wait: serde_json::Value,
) -> Result<StopResponse, TransportError> {
    let params = vec![json!(wait)];
    let raw = transport.send_request("stop", &params).await?;
    Ok(serde_json::from_value::<StopResponse>(raw)?)
}
