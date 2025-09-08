//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Returns the total amount received by the given address in transactions with at least minconf confirmations.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getreceivedbyaddress;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getreceivedbyaddress(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns the total amount received by the given address in transactions with at least minconf confirmations.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetreceivedbyaddressResponse(pub f64);

/// Calls the `getreceivedbyaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getreceivedbyaddress(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    minconf: serde_json::Value,
    include_immature_coinbase: serde_json::Value,
) -> Result<GetreceivedbyaddressResponse, TransportError> {
    let params = vec![
        json!(address),
        json!(minconf),
        json!(include_immature_coinbase),
    ];
    let raw = transport
        .send_request("getreceivedbyaddress", &params)
        .await?;
    Ok(serde_json::from_value::<GetreceivedbyaddressResponse>(raw)?)
}
