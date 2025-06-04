//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Stops current wallet rescan triggered by an RPC call, e.g. by an importprivkey call.
/// Note: Use "getwalletinfo" to query the scanning progress.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::abortrescan;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.abortrescan().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `abortrescan` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AbortrescanResponse {
    /// Whether the abort was successful
    pub field_0: bool,
}



/// Calls the `abortrescan` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn abortrescan(transport: &dyn Transport) -> Result<AbortrescanResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("abortrescan", &params).await?;
    Ok(serde_json::from_value::<AbortrescanResponse>(raw)?)
}
