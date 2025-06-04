//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Encrypts the wallet with 'passphrase'. This is for first time encryption.
/// After this, any calls that interact with private keys such as sending or signing
/// will require the passphrase to be set prior the making these calls.
/// Use the walletpassphrase call for this, and then walletlock call.
/// If the wallet is already encrypted, use the walletpassphrasechange call.
/// ** IMPORTANT **
/// For security reasons, the encryption process will generate a new HD seed, resulting
/// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
/// securely back up the newly generated wallet file using the backupwallet RPC.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::encryptwallet;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.encryptwallet(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `encryptwallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EncryptwalletResponse {
    /// A string with further instructions
    pub field_0: String,
}



/// Calls the `encryptwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn encryptwallet(transport: &dyn Transport, passphrase: serde_json::Value) -> Result<EncryptwalletResponse, TransportError> {
    let params = vec![json!(passphrase)];
    let raw = transport.send_request("encryptwallet", &params).await?;
    Ok(serde_json::from_value::<EncryptwalletResponse>(raw)?)
}
