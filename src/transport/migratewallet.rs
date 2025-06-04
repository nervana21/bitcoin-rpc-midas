//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Migrate the wallet to a descriptor wallet.
/// A new wallet backup will need to be made.
/// The migration process will create a backup of the wallet before migrating. This backup
/// file will be named <wallet name>-<timestamp>.legacy.bak and can be found in the directory
/// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
/// Encrypted wallets must have the passphrase provided as an argument to this call.
/// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::migratewallet;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.migratewallet(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `migratewallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MigratewalletResponse {
    /// The name of the primary migrated wallet
    pub wallet_name: String,
    /// The name of the migrated wallet containing the watchonly scripts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchonly_name: Option<String>,
    /// The name of the migrated wallet containing solvable but not watched scripts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solvables_name: Option<String>,
    /// The location of the backup of the original wallet
    pub backup_path: String,
}



/// Calls the `migratewallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn migratewallet(transport: &dyn Transport, wallet_name: serde_json::Value, passphrase: serde_json::Value) -> Result<MigratewalletResponse, TransportError> {
    let params = vec![json!(wallet_name), json!(passphrase)];
    let raw = transport.send_request("migratewallet", &params).await?;
    Ok(serde_json::from_value::<MigratewalletResponse>(raw)?)
}
