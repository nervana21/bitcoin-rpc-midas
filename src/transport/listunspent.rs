//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::listunspent;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.listunspent(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListunspentResponse(pub Vec<serde_json::Value>);

/// Calls the `listunspent` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listunspent(
    transport: &dyn TransportTrait,
    minconf: serde_json::Value,
    maxconf: serde_json::Value,
    addresses: serde_json::Value,
    include_unsafe: serde_json::Value,
    query_options: serde_json::Value,
) -> Result<ListunspentResponse, TransportError> {
    let params = vec![
        json!(minconf),
        json!(maxconf),
        json!(addresses),
        json!(include_unsafe),
        json!(query_options),
    ];
    let raw = transport.send_request("listunspent", &params).await?;
    Ok(serde_json::from_value::<ListunspentResponse>(raw)?)
}
