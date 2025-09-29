//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Encrypts the wallet with 'passphrase'. This is for first time encryption.
/// After this, any calls that interact with private keys such as sending or signing
/// will require the passphrase to be set prior to making these calls.
/// Use the walletpassphrase call for this, and then walletlock call.
/// If the wallet is already encrypted, use the walletpassphrasechange call.
/// ** IMPORTANT **
/// For security reasons, the encryption process will generate a new HD seed, resulting
/// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
/// securely back up the newly generated wallet file using the backupwallet RPC.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.encryptwallet(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::encryptwallet;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = encryptwallet(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Encrypts the wallet with 'passphrase'. This is for first time encryption.
/// After this, any calls that interact with private keys such as sending or signing
/// will require the passphrase to be set prior to making these calls.
/// Use the walletpassphrase call for this, and then walletlock call.
/// If the wallet is already encrypted, use the walletpassphrasechange call.
/// ** IMPORTANT **
/// For security reasons, the encryption process will generate a new HD seed, resulting
/// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
/// securely back up the newly generated wallet file using the backupwallet RPC.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EncryptwalletResponse(pub String);

/// Calls the `encryptwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn encryptwallet(
    transport: &dyn TransportTrait,
    passphrase: serde_json::Value,
) -> Result<EncryptwalletResponse, TransportError> {
    let params = vec![json!(passphrase)];
    let raw = transport.send_request("encryptwallet", &params).await?;
    Ok(serde_json::from_value::<EncryptwalletResponse>(raw)?)
}
