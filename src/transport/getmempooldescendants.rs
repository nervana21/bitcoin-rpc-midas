//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// If txid is in the mempool, returns all in-mempool descendants.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getmempooldescendants;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getmempooldescendants(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// If txid is in the mempool, returns all in-mempool descendants.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmempooldescendantsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactionid: Option<serde_json::Value>,
}

/// Calls the `getmempooldescendants` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmempooldescendants(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<GetmempooldescendantsResponse, TransportError> {
    let params = vec![json!(txid), json!(verbose)];
    let raw = transport
        .send_request("getmempooldescendants", &params)
        .await?;
    Ok(serde_json::from_value::<GetmempooldescendantsResponse>(
        raw,
    )?)
}
