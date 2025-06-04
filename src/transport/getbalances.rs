//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns an object with all balances in BTC.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getbalances;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getbalances().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getbalances` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbalancesResponse {
    /// balances from outputs that the wallet can sign
    pub mine: serde_json::Value,
    /// watchonly balances (not present if wallet does not watch anything)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchonly: Option<serde_json::Value>,
    /// hash and height of the block this information was generated on
    pub lastprocessedblock: bitcoin::Block,
}



/// Calls the `getbalances` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getbalances(transport: &dyn Transport) -> Result<GetbalancesResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getbalances", &params).await?;
    Ok(serde_json::from_value::<GetbalancesResponse>(raw)?)
}
