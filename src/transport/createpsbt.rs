//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::createpsbt;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.createpsbt(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreatepsbtResponse(pub String);

/// Calls the `createpsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn createpsbt(
    transport: &dyn TransportTrait,
    inputs: serde_json::Value,
    outputs: serde_json::Value,
    locktime: serde_json::Value,
    replaceable: serde_json::Value,
) -> Result<CreatepsbtResponse, TransportError> {
    let params = vec![
        json!(inputs),
        json!(outputs),
        json!(locktime),
        json!(replaceable),
    ];
    let raw = transport.send_request("createpsbt", &params).await?;
    Ok(serde_json::from_value::<CreatepsbtResponse>(raw)?)
}
