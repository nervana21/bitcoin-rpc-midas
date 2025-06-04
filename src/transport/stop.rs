//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Request a graceful shutdown of Bitcoin Core.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::stop;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.stop(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `stop` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StopResponse {
    /// A string with the content 'Bitcoin Core stopping'
    pub field_0: String,
}



/// Calls the `stop` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn stop(transport: &dyn Transport, wait: serde_json::Value) -> Result<StopResponse, TransportError> {
    let params = vec![json!(wait)];
    let raw = transport.send_request("stop", &params).await?;
    Ok(serde_json::from_value::<StopResponse>(raw)?)
}
