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
use transport::{TransportTrait, TransportError};
/// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
    /// No input in any of the PSBTs can be in more than one of the PSBTs.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct JoinpsbtsResponse(pub String);



/// Calls the `joinpsbts` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn joinpsbts(transport: &dyn TransportTrait, txs: serde_json::Value) -> Result<JoinpsbtsResponse, TransportError> {
    let params = vec![json!(txs)];
    let raw = transport.send_request("joinpsbts", &params).await?;
    Ok(serde_json::from_value::<JoinpsbtsResponse>(raw)?)
}
