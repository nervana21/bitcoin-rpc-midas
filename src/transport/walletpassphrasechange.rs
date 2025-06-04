//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Changes the wallet passphrase from 'oldpassphrase' to 'newpassphrase'.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::walletpassphrasechange;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.walletpassphrasechange(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};


/// Calls the `walletpassphrasechange` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn walletpassphrasechange(transport: &dyn Transport, oldpassphrase: serde_json::Value, newpassphrase: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(oldpassphrase), json!(newpassphrase)];
    let raw = transport.send_request("walletpassphrasechange", &params).await?;
    Ok(raw)
}
