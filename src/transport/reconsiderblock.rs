//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Removes invalidity status of a block, its ancestors and its descendants, reconsider them for activation.
/// This can be used to undo the effects of invalidateblock.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::reconsiderblock;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.reconsiderblock(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};


/// Calls the `reconsiderblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn reconsiderblock(transport: &dyn Transport, blockhash: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(blockhash)];
    let raw = transport.send_request("reconsiderblock", &params).await?;
    Ok(raw)
}
