//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
/// Throws when the header is invalid.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::submitheader;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.submitheader(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};


/// Calls the `submitheader` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn submitheader(transport: &dyn Transport, hexdata: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(hexdata)];
    let raw = transport.send_request("submitheader", &params).await?;
    Ok(raw)
}
