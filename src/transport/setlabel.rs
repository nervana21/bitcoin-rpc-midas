//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Sets the label associated with the given address.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::setlabel;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.setlabel(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `setlabel` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn setlabel(transport: &dyn TransportTrait, address: serde_json::Value, label: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(address), json!(label)];
    let raw = transport.send_request("setlabel", &params).await?;
    Ok(raw)
}
