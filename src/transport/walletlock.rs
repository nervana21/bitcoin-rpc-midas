//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Removes the wallet encryption key from memory, locking the wallet.
/// After calling this method, you will need to call walletpassphrase again
/// before being able to call any methods which require the wallet to be unlocked.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::walletlock;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.walletlock().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};

/// Calls the `walletlock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn walletlock(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("walletlock", &params).await?;
    Ok(raw)
}
