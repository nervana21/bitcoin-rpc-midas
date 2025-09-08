//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
/// Note: Use "getwalletinfo" to query the scanning progress.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::abortrescan;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.abortrescan().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
/// Note: Use \"getwalletinfo\" to query the scanning progress.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AbortrescanResponse(pub bool);

/// Calls the `abortrescan` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn abortrescan(
    transport: &dyn TransportTrait,
) -> Result<AbortrescanResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("abortrescan", &params).await?;
    Ok(serde_json::from_value::<AbortrescanResponse>(raw)?)
}
