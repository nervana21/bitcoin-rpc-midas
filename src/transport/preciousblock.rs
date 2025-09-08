//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Treats a block as if it were received before others with the same work.
/// A later preciousblock call can override the effect of an earlier one.
/// The effects of preciousblock are not retained across restarts.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::preciousblock;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.preciousblock(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};

/// Calls the `preciousblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn preciousblock(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(blockhash)];
    let raw = transport.send_request("preciousblock", &params).await?;
    Ok(raw)
}
