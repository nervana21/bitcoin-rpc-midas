//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::getnettotals;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getnettotals().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetnettotalsResponse {
    pub totalbytesrecv: u64,
    pub totalbytessent: u64,
    pub timemillis: serde_json::Value,
    pub uploadtarget: serde_json::Value,
}

/// Calls the `getnettotals` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getnettotals(
    transport: &dyn TransportTrait,
) -> Result<GetnettotalsResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getnettotals", &params).await?;
    Ok(serde_json::from_value::<GetnettotalsResponse>(raw)?)
}
