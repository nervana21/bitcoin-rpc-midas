//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Unloads the wallet referenced by the request endpoint, otherwise unloads the wallet specified in the argument.
/// Specifying the wallet name on a wallet endpoint is invalid.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::unloadwallet;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.unloadwallet(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `unloadwallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UnloadwalletResponse {
    /// Warning messages, if any, related to unloading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}



/// Calls the `unloadwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn unloadwallet(transport: &dyn Transport, wallet_name: serde_json::Value, load_on_startup: serde_json::Value) -> Result<UnloadwalletResponse, TransportError> {
    let params = vec![json!(wallet_name), json!(load_on_startup)];
    let raw = transport.send_request("unloadwallet", &params).await?;
    Ok(serde_json::from_value::<UnloadwalletResponse>(raw)?)
}
