//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns a list of external signers from -signer.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::enumeratesigners;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.enumeratesigners().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `enumeratesigners` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EnumeratesignersResponse {
    pub signers: Vec<serde_json::Value>,
}



/// Calls the `enumeratesigners` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn enumeratesigners(transport: &dyn Transport) -> Result<EnumeratesignersResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("enumeratesigners", &params).await?;
    Ok(serde_json::from_value::<EnumeratesignersResponse>(raw)?)
}
