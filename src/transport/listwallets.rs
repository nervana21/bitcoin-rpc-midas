//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use "getwalletinfo"

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::listwallets;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.listwallets().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `listwallets` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListwalletsResponse {
    pub field_0: Vec<serde_json::Value>,
}



/// Calls the `listwallets` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listwallets(transport: &dyn Transport) -> Result<ListwalletsResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listwallets", &params).await?;
    Ok(serde_json::from_value::<ListwalletsResponse>(raw)?)
}
