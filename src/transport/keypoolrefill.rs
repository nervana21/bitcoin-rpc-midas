//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Fills the keypool.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::keypoolrefill;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.keypoolrefill(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};


/// Calls the `keypoolrefill` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn keypoolrefill(transport: &dyn Transport, newsize: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(newsize)];
    let raw = transport.send_request("keypoolrefill", &params).await?;
    Ok(raw)
}
