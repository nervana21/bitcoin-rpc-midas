//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Treats a block as if it were received before others with the same work.
/// A later preciousblock call can override the effect of an earlier one.
/// The effects of preciousblock are not retained across restarts.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::preciousblock;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.preciousblock(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};


/// Calls the `preciousblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn preciousblock(transport: &dyn Transport, blockhash: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(blockhash)];
    let raw = transport.send_request("preciousblock", &params).await?;
    Ok(raw)
}
