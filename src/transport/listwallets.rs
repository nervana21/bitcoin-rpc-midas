//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use "getwalletinfo"

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::listwallets;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.listwallets().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use \"getwalletinfo\"
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListwalletsResponse(pub Vec<serde_json::Value>);

/// Calls the `listwallets` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listwallets(
    transport: &dyn TransportTrait,
) -> Result<ListwalletsResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listwallets", &params).await?;
    Ok(serde_json::from_value::<ListwalletsResponse>(raw)?)
}
