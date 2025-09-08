//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Accepts the transaction into mined blocks at a higher (or lower) priority

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::prioritisetransaction;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.prioritisetransaction(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Accepts the transaction into mined blocks at a higher (or lower) priority
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PrioritisetransactionResponse(pub bool);

/// Calls the `prioritisetransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn prioritisetransaction(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    dummy: serde_json::Value,
    fee_delta: serde_json::Value,
) -> Result<PrioritisetransactionResponse, TransportError> {
    let params = vec![json!(txid), json!(dummy), json!(fee_delta)];
    let raw = transport
        .send_request("prioritisetransaction", &params)
        .await?;
    Ok(serde_json::from_value::<PrioritisetransactionResponse>(
        raw,
    )?)
}
