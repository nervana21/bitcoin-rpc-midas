//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::backupwallet;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.backupwallet(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};

/// Calls the `backupwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn backupwallet(
    transport: &dyn TransportTrait,
    destination: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(destination)];
    let raw = transport.send_request("backupwallet", &params).await?;
    Ok(raw)
}
