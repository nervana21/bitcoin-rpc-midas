//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::sendmany;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.sendmany(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
pub struct SendmanyResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_reason: Option<String>,
}

/// Calls the `sendmany` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn sendmany(
    transport: &dyn TransportTrait,
    dummy: serde_json::Value,
    amounts: serde_json::Value,
    minconf: serde_json::Value,
    comment: serde_json::Value,
    subtractfeefrom: serde_json::Value,
    replaceable: serde_json::Value,
    conf_target: serde_json::Value,
    estimate_mode: serde_json::Value,
    fee_rate: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<SendmanyResponse, TransportError> {
    let params = vec![
        json!(dummy),
        json!(amounts),
        json!(minconf),
        json!(comment),
        json!(subtractfeefrom),
        json!(replaceable),
        json!(conf_target),
        json!(estimate_mode),
        json!(fee_rate),
        json!(verbose),
    ];
    let raw = transport.send_request("sendmany", &params).await?;
    Ok(serde_json::from_value::<SendmanyResponse>(raw)?)
}
