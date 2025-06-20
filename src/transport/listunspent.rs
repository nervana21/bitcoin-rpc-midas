//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::listunspent;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.listunspent(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns array of unspent transaction outputs
    /// with between minconf and maxconf (inclusive) confirmations.
    /// Optionally filter to only include txouts paid to specified addresses.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListunspentResponse(pub Vec<serde_json::Value>);



/// Calls the `listunspent` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listunspent(transport: &dyn TransportTrait, minconf: serde_json::Value, maxconf: serde_json::Value, addresses: serde_json::Value, include_unsafe: serde_json::Value, query_options: serde_json::Value) -> Result<ListunspentResponse, TransportError> {
    let params = vec![json!(minconf), json!(maxconf), json!(addresses), json!(include_unsafe), json!(query_options)];
    let raw = transport.send_request("listunspent", &params).await?;
    Ok(serde_json::from_value::<ListunspentResponse>(raw)?)
}
