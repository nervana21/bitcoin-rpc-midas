//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Immediately disconnects from the specified peer node.
/// Strictly one out of 'address' and 'nodeid' can be provided to identify the node.
/// To disconnect by nodeid, either set 'address' to the empty string, or call using the named 'nodeid' argument only.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::disconnectnode;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.disconnectnode(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};

/// Calls the `disconnectnode` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn disconnectnode(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    nodeid: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(address), json!(nodeid)];
    let raw = transport.send_request("disconnectnode", &params).await?;
    Ok(raw)
}
