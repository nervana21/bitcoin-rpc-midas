//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Returns a list of wallets in the wallet directory.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::listwalletdir;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.listwalletdir().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns a list of wallets in the wallet directory.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListwalletdirResponse {
    pub wallets: Vec<String>,
}

/// Calls the `listwalletdir` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listwalletdir(
    transport: &dyn TransportTrait,
) -> Result<ListwalletdirResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listwalletdir", &params).await?;
    Ok(serde_json::from_value::<ListwalletdirResponse>(raw)?)
}
