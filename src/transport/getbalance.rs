//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::getbalance;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getbalance(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetbalanceResponse(pub f64);

/// Calls the `getbalance` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getbalance(
    transport: &dyn TransportTrait,
    dummy: serde_json::Value,
    minconf: serde_json::Value,
    include_watchonly: serde_json::Value,
    avoid_reuse: serde_json::Value,
) -> Result<GetbalanceResponse, TransportError> {
    let params = vec![
        json!(dummy),
        json!(minconf),
        json!(include_watchonly),
        json!(avoid_reuse),
    ];
    let raw = transport.send_request("getbalance", &params).await?;
    Ok(serde_json::from_value::<GetbalanceResponse>(raw)?)
}
