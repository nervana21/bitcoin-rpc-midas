//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
/// Returns up to 'count' most recent transactions skipping the first 'from' transactions.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::listtransactions;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.listtransactions(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
///
/// Returns up to 'count' most recent transactions skipping the first 'from' transactions.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListtransactionsResponse(pub Vec<serde_json::Value>);

/// Calls the `listtransactions` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listtransactions(
    transport: &dyn TransportTrait,
    label: serde_json::Value,
    count: serde_json::Value,
    skip: serde_json::Value,
    include_watchonly: serde_json::Value,
) -> Result<ListtransactionsResponse, TransportError> {
    let params = vec![
        json!(label),
        json!(count),
        json!(skip),
        json!(include_watchonly),
    ];
    let raw = transport.send_request("listtransactions", &params).await?;
    Ok(serde_json::from_value::<ListtransactionsResponse>(raw)?)
}
