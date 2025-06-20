//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getdifficulty;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getdifficulty().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetdifficultyResponse(pub f64);



/// Calls the `getdifficulty` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getdifficulty(transport: &dyn TransportTrait) -> Result<GetdifficultyResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getdifficulty", &params).await?;
    Ok(serde_json::from_value::<GetdifficultyResponse>(raw)?)
}
