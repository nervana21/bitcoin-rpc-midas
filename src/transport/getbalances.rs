//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Returns an object with all balances in BTC.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::getbalances;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getbalances().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns an object with all balances in BTC.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetbalancesResponse {
    pub mine: serde_json::Value,
    pub lastprocessedblock: serde_json::Value,
}

/// Calls the `getbalances` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getbalances(
    transport: &dyn TransportTrait,
) -> Result<GetbalancesResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getbalances", &params).await?;
    Ok(serde_json::from_value::<GetbalancesResponse>(raw)?)
}
