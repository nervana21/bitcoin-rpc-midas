//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// DEPRECATED
/// Identical to getbalances().mine.untrusted_pending

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getunconfirmedbalance;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getunconfirmedbalance().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// DEPRECATED
    /// Identical to getbalances().mine.untrusted_pending
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetunconfirmedbalanceResponse(pub bitcoin::Amount);



/// Calls the `getunconfirmedbalance` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getunconfirmedbalance(transport: &dyn TransportTrait) -> Result<GetunconfirmedbalanceResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getunconfirmedbalance", &params).await?;
    Ok(serde_json::from_value::<GetunconfirmedbalanceResponse>(raw)?)
}
