//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns details of the RPC server.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getrpcinfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getrpcinfo().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getrpcinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrpcinfoResponse {
    /// All active commands
    pub active_commands: Vec<serde_json::Value>,
    /// The complete file path to the debug log
    pub logpath: String,
}



/// Calls the `getrpcinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrpcinfo(transport: &dyn Transport) -> Result<GetrpcinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getrpcinfo", &params).await?;
    Ok(serde_json::from_value::<GetrpcinfoResponse>(raw)?)
}
