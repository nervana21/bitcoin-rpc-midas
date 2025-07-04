//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::backupwallet;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.backupwallet(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `backupwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn backupwallet(transport: &dyn TransportTrait, destination: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(destination)];
    let raw = transport.send_request("backupwallet", &params).await?;
    Ok(raw)
}
