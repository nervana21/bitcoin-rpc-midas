//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getreceivedbylabel;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getreceivedbylabel(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetreceivedbylabelResponse(pub f64);

/// Calls the `getreceivedbylabel` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getreceivedbylabel(
    transport: &dyn TransportTrait,
    label: serde_json::Value,
    minconf: serde_json::Value,
    include_immature_coinbase: serde_json::Value,
) -> Result<GetreceivedbylabelResponse, TransportError> {
    let params = vec![
        json!(label),
        json!(minconf),
        json!(include_immature_coinbase),
    ];
    let raw = transport
        .send_request("getreceivedbylabel", &params)
        .await?;
    Ok(serde_json::from_value::<GetreceivedbylabelResponse>(raw)?)
}
