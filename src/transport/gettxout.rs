//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns details about an unspent transaction output.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::gettxout;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.gettxout(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns details about an unspent transaction output.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GettxoutResponse(pub serde_json::Value);

/// Calls the `gettxout` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettxout(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    n: serde_json::Value,
    include_mempool: serde_json::Value,
) -> Result<GettxoutResponse, TransportError> {
    let params = vec![json!(txid), json!(n), json!(include_mempool)];
    let raw = transport.send_request("gettxout", &params).await?;
    Ok(serde_json::from_value::<GettxoutResponse>(raw)?)
}
