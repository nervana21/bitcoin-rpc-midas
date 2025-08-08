//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Requests that a ping be sent to all other nodes, to measure ping time.
/// Results are provided in getpeerinfo.
/// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::ping;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.ping().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `ping` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn ping(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("ping", &params).await?;
    Ok(raw)
}
