//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Waits for the validation interface queue to catch up on everything that was there when we entered this function.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::syncwithvalidationinterfacequeue;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.syncwithvalidationinterfacequeue().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `syncwithvalidationinterfacequeue` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn syncwithvalidationinterfacequeue(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("syncwithvalidationinterfacequeue", &params).await?;
    Ok(raw)
}
