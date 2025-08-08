//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns information about the active ZeroMQ notifications.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getzmqnotifications;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getzmqnotifications().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns information about the active ZeroMQ notifications.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetzmqnotificationsResponse(pub Vec<serde_json::Value>);



/// Calls the `getzmqnotifications` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getzmqnotifications(transport: &dyn TransportTrait) -> Result<GetzmqnotificationsResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getzmqnotifications", &params).await?;
    Ok(serde_json::from_value::<GetzmqnotificationsResponse>(raw)?)
}
