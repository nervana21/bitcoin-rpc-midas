//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Return JSON description of RPC API.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::api;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.api().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Return JSON description of RPC API.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ApiResponse(pub serde_json::Value);



/// Calls the `api` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn api(transport: &dyn TransportTrait) -> Result<ApiResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("api", &params).await?;
    Ok(serde_json::from_value::<ApiResponse>(raw)?)
}
