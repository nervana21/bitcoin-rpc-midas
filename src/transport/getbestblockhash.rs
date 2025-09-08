//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Returns the hash of the best (tip) block in the most-work fully-validated chain.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getbestblockhash;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getbestblockhash().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns the hash of the best (tip) block in the most-work fully-validated chain.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetbestblockhashResponse(pub bitcoin::BlockHash);

/// Calls the `getbestblockhash` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getbestblockhash(
    transport: &dyn TransportTrait,
) -> Result<GetbestblockhashResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getbestblockhash", &params).await?;
    Ok(serde_json::from_value::<GetbestblockhashResponse>(raw)?)
}
