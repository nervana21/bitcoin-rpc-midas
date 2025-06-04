//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Returns information on all address manager entries for the new and tried tables.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getrawaddrman;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getrawaddrman().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getrawaddrman` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrawaddrmanResponse {
    pub field_0: serde_json::Value,
}



/// Calls the `getrawaddrman` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrawaddrman(transport: &dyn Transport) -> Result<GetrawaddrmanResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getrawaddrman", &params).await?;
    Ok(serde_json::from_value::<GetrawaddrmanResponse>(raw)?)
}
