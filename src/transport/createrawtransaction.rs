//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::createrawtransaction;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.createrawtransaction(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreaterawtransactionResponse(pub String);

/// Calls the `createrawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn createrawtransaction(
    transport: &dyn TransportTrait,
    inputs: serde_json::Value,
    outputs: serde_json::Value,
    locktime: serde_json::Value,
    replaceable: serde_json::Value,
) -> Result<CreaterawtransactionResponse, TransportError> {
    let params = vec![
        json!(inputs),
        json!(outputs),
        json!(locktime),
        json!(replaceable),
    ];
    let raw = transport
        .send_request("createrawtransaction", &params)
        .await?;
    Ok(serde_json::from_value::<CreaterawtransactionResponse>(raw)?)
}
