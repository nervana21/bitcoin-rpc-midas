//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Migrate the wallet to a descriptor wallet.
/// A new wallet backup will need to be made.
/// The migration process will create a backup of the wallet before migrating. This backup
/// file will be named <wallet name>-<timestamp>.legacy.bak and can be found in the directory
/// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
/// Encrypted wallets must have the passphrase provided as an argument to this call.
/// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.migratewallet(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::migratewallet;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = migratewallet(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Migrate the wallet to a descriptor wallet.
/// A new wallet backup will need to be made.
///
/// The migration process will create a backup of the wallet before migrating. This backup
/// file will be named <wallet name>-<timestamp>.legacy.bak and can be found in the directory
/// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
/// Encrypted wallets must have the passphrase provided as an argument to this call.
///
/// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
#[derive(Debug, Deserialize, Serialize)]
pub struct MigratewalletResponse {
    pub wallet_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchonly_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solvables_name: Option<String>,
    pub backup_path: String,
}

/// Calls the `migratewallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn migratewallet(
    transport: &dyn TransportTrait,
    wallet_name: serde_json::Value,
    passphrase: serde_json::Value,
) -> Result<MigratewalletResponse, TransportError> {
    let params = vec![json!(wallet_name), json!(passphrase)];
    let raw = transport.send_request("migratewallet", &params).await?;
    Ok(serde_json::from_value::<MigratewalletResponse>(raw)?)
}
