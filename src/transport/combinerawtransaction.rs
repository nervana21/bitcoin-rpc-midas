//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::combinerawtransaction;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.combinerawtransaction(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CombinerawtransactionResponse(pub String);

/// Calls the `combinerawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn combinerawtransaction(
    transport: &dyn TransportTrait,
    txs: serde_json::Value,
) -> Result<CombinerawtransactionResponse, TransportError> {
    let params = vec![json!(txs)];
    let raw = transport
        .send_request("combinerawtransaction", &params)
        .await?;
    Ok(serde_json::from_value::<CombinerawtransactionResponse>(
        raw,
    )?)
}
