//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::simulaterawtransaction;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.simulaterawtransaction(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
#[derive(Debug, Deserialize, Serialize)]
pub struct SimulaterawtransactionResponse {
    pub balance_change: serde_json::Value,
}



/// Calls the `simulaterawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn simulaterawtransaction(transport: &dyn TransportTrait, rawtxs: serde_json::Value, options: serde_json::Value) -> Result<SimulaterawtransactionResponse, TransportError> {
    let params = vec![json!(rawtxs), json!(options)];
    let raw = transport.send_request("simulaterawtransaction", &params).await?;
    Ok(serde_json::from_value::<SimulaterawtransactionResponse>(raw)?)
}
