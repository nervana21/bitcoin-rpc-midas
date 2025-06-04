//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Stores the wallet decryption key in memory for 'timeout' seconds.
/// This is needed prior to performing transactions related to private keys such as sending bitcoins
/// Note:
/// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
/// time that overrides the old one.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::walletpassphrase;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.walletpassphrase(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};


/// Calls the `walletpassphrase` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn walletpassphrase(transport: &dyn Transport, passphrase: serde_json::Value, timeout: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(passphrase), json!(timeout)];
    let raw = transport.send_request("walletpassphrase", &params).await?;
    Ok(raw)
}
