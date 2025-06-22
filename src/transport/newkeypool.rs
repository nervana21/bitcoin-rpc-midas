//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Entirely clears and refills the keypool.
/// WARNING: On non-HD wallets, this will require a new backup immediately, to include the new keys.
/// When restoring a backup of an HD wallet created before the newkeypool command is run, funds received to
/// new addresses may not appear automatically. They have not been lost, but the wallet may not find them.
/// This can be fixed by running the newkeypool command on the backup and then rescanning, so the wallet
/// re-generates the required keys.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::newkeypool;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.newkeypool().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `newkeypool` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn newkeypool(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("newkeypool", &params).await?;
    Ok(raw)
}
