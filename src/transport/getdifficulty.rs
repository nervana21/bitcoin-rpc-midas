//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getdifficulty;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getdifficulty().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetdifficultyResponse(pub f64);

/// Calls the `getdifficulty` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getdifficulty(
    transport: &dyn TransportTrait,
) -> Result<GetdifficultyResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getdifficulty", &params).await?;
    Ok(serde_json::from_value::<GetdifficultyResponse>(raw)?)
}
