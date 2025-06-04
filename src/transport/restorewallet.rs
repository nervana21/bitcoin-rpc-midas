//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Restores and loads a wallet from backup.
/// The rescan is significantly faster if a descriptor wallet is restored
/// and block filters are available (using startup option "-blockfilterindex=1").

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::restorewallet;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.restorewallet(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `restorewallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RestorewalletResponse {
    /// The wallet name if restored successfully.
    pub name: String,
    /// Warning messages, if any, related to restoring and loading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}



/// Calls the `restorewallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn restorewallet(transport: &dyn Transport, wallet_name: serde_json::Value, backup_file: serde_json::Value, load_on_startup: serde_json::Value) -> Result<RestorewalletResponse, TransportError> {
    let params = vec![json!(wallet_name), json!(backup_file), json!(load_on_startup)];
    let raw = transport.send_request("restorewallet", &params).await?;
    Ok(serde_json::from_value::<RestorewalletResponse>(raw)?)
}
