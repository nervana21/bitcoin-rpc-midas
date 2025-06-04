//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::sendmsgtopeer;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.sendmsgtopeer(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};


/// Calls the `sendmsgtopeer` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn sendmsgtopeer(transport: &dyn Transport, peer_id: serde_json::Value, msg_type: serde_json::Value, msg: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(peer_id), json!(msg_type), json!(msg)];
    let raw = transport.send_request("sendmsgtopeer", &params).await?;
    Ok(raw)
}
