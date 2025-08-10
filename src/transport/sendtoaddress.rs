//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::sendtoaddress;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.sendtoaddress(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
pub struct SendtoaddressResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_reason: Option<String>,
}

/// Calls the `sendtoaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn sendtoaddress(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    amount: serde_json::Value,
    comment: serde_json::Value,
    comment_to: serde_json::Value,
    subtractfeefromamount: serde_json::Value,
    replaceable: serde_json::Value,
    conf_target: serde_json::Value,
    estimate_mode: serde_json::Value,
    avoid_reuse: serde_json::Value,
    fee_rate: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<SendtoaddressResponse, TransportError> {
    let params = vec![
        json!(address),
        json!(amount),
        json!(comment),
        json!(comment_to),
        json!(subtractfeefromamount),
        json!(replaceable),
        json!(conf_target),
        json!(estimate_mode),
        json!(avoid_reuse),
        json!(fee_rate),
        json!(verbose),
    ];
    let raw = transport.send_request("sendtoaddress", &params).await?;
    Ok(serde_json::from_value::<SendtoaddressResponse>(raw)?)
}
