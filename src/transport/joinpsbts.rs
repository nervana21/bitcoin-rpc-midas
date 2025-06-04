//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
/// No input in any of the PSBTs can be in more than one of the PSBTs.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::joinpsbts;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.joinpsbts(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `joinpsbts` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct JoinpsbtsResponse {
    /// The base64-encoded partially signed transaction
    pub field_0: String,
}



/// Calls the `joinpsbts` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn joinpsbts(transport: &dyn Transport, txs: serde_json::Value) -> Result<JoinpsbtsResponse, TransportError> {
    let params = vec![json!(txs)];
    let raw = transport.send_request("joinpsbts", &params).await?;
    Ok(serde_json::from_value::<JoinpsbtsResponse>(raw)?)
}
