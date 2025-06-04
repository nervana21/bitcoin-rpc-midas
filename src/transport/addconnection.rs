//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Open an outbound connection to a specified node. This RPC is for testing only.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::addconnection;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.addconnection(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `addconnection` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddconnectionResponse {
    /// Address of newly added connection.
    pub address: String,
    /// Type of connection opened.
    pub connection_type: String,
}



/// Calls the `addconnection` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn addconnection(transport: &dyn Transport, address: serde_json::Value, connection_type: serde_json::Value, v2transport: serde_json::Value) -> Result<AddconnectionResponse, TransportError> {
    let params = vec![json!(address), json!(connection_type), json!(v2transport)];
    let raw = transport.send_request("addconnection", &params).await?;
    Ok(serde_json::from_value::<AddconnectionResponse>(raw)?)
}
