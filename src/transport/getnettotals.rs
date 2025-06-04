//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getnettotals;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getnettotals().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getnettotals` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnettotalsResponse {
    /// Total bytes received
    pub totalbytesrecv: u64,
    /// Total bytes sent
    pub totalbytessent: u64,
    /// Current system UNIX epoch time in milliseconds
    pub timemillis: serde_json::Value,
    pub uploadtarget: serde_json::Value,
}



/// Calls the `getnettotals` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getnettotals(transport: &dyn Transport) -> Result<GetnettotalsResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getnettotals", &params).await?;
    Ok(serde_json::from_value::<GetnettotalsResponse>(raw)?)
}
