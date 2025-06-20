//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Mine to a specified descriptor and return the block hashes.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::generatetodescriptor;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.generatetodescriptor(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Mine to a specified descriptor and return the block hashes.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GeneratetodescriptorResponse(pub Vec<serde_json::Value>);



/// Calls the `generatetodescriptor` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn generatetodescriptor(transport: &dyn TransportTrait, num_blocks: serde_json::Value, descriptor: serde_json::Value, maxtries: serde_json::Value) -> Result<GeneratetodescriptorResponse, TransportError> {
    let params = vec![json!(num_blocks), json!(descriptor), json!(maxtries)];
    let raw = transport.send_request("generatetodescriptor", &params).await?;
    Ok(serde_json::from_value::<GeneratetodescriptorResponse>(raw)?)
}
