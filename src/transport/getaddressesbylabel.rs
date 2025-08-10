//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Returns the list of addresses assigned the specified label.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::getaddressesbylabel;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getaddressesbylabel(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns the list of addresses assigned the specified label.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetaddressesbylabelResponse {
    pub address: serde_json::Value,
}

/// Calls the `getaddressesbylabel` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getaddressesbylabel(
    transport: &dyn TransportTrait,
    label: serde_json::Value,
) -> Result<GetaddressesbylabelResponse, TransportError> {
    let params = vec![json!(label)];
    let raw = transport
        .send_request("getaddressesbylabel", &params)
        .await?;
    Ok(serde_json::from_value::<GetaddressesbylabelResponse>(raw)?)
}
