//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::combinerawtransaction;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.combinerawtransaction(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `combinerawtransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CombinerawtransactionResponse {
    /// The hex-encoded raw transaction with signature(s)
    pub field_0: String,
}



/// Calls the `combinerawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn combinerawtransaction(transport: &dyn Transport, txs: serde_json::Value) -> Result<CombinerawtransactionResponse, TransportError> {
    let params = vec![json!(txs)];
    let raw = transport.send_request("combinerawtransaction", &params).await?;
    Ok(serde_json::from_value::<CombinerawtransactionResponse>(raw)?)
}
