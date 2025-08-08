//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Attempts to add or remove a node from the addnode list.
/// Or try a connection to a node once.
/// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
/// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
/// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::addnode;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.addnode(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};

/// Calls the `addnode` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn addnode(
    transport: &dyn TransportTrait,
    node: serde_json::Value,
    command: serde_json::Value,
    v2transport: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(node), json!(command), json!(v2transport)];
    let raw = transport.send_request("addnode", &params).await?;
    Ok(raw)
}
