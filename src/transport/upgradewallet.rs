//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Upgrade the wallet. Upgrades to the latest version if no version number is specified.
/// New keys may be generated and a new wallet backup will need to be made.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::upgradewallet;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.upgradewallet(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `upgradewallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradewalletResponse {
    /// Name of wallet this operation was performed on
    pub wallet_name: String,
    /// Version of wallet before this operation
    pub previous_version: u64,
    /// Version of wallet after this operation
    pub current_version: u64,
    /// Description of result, if no error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// Error message (if there is one)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}



/// Calls the `upgradewallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn upgradewallet(transport: &dyn Transport, version: serde_json::Value) -> Result<UpgradewalletResponse, TransportError> {
    let params = vec![json!(version)];
    let raw = transport.send_request("upgradewallet", &params).await?;
    Ok(serde_json::from_value::<UpgradewalletResponse>(raw)?)
}
