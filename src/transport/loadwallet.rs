//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::loadwallet;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.loadwallet(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct LoadwalletResponse {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}

/// Calls the `loadwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn loadwallet(
    transport: &dyn TransportTrait,
    filename: serde_json::Value,
    load_on_startup: serde_json::Value,
) -> Result<LoadwalletResponse, TransportError> {
    let params = vec![json!(filename), json!(load_on_startup)];
    let raw = transport.send_request("loadwallet", &params).await?;
    Ok(serde_json::from_value::<LoadwalletResponse>(raw)?)
}
