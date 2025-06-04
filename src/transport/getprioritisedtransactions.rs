//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getprioritisedtransactions;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getprioritisedtransactions().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getprioritisedtransactions` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetprioritisedtransactionsResponse {
    /// prioritisation keyed by txid
    pub field_0: serde_json::Value,
}



/// Calls the `getprioritisedtransactions` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getprioritisedtransactions(transport: &dyn Transport) -> Result<GetprioritisedtransactionsResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getprioritisedtransactions", &params).await?;
    Ok(serde_json::from_value::<GetprioritisedtransactionsResponse>(raw)?)
}
