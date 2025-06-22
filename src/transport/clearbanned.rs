//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Clear all banned IPs.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::clearbanned;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.clearbanned().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `clearbanned` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn clearbanned(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("clearbanned", &params).await?;
    Ok(raw)
}
