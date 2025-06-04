//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getaddrmaninfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getaddrmaninfo().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getaddrmaninfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetaddrmaninfoResponse {
    /// json object with network type as keys
    pub field_0: serde_json::Value,
}



/// Calls the `getaddrmaninfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getaddrmaninfo(transport: &dyn Transport) -> Result<GetaddrmaninfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getaddrmaninfo", &params).await?;
    Ok(serde_json::from_value::<GetaddrmaninfoResponse>(raw)?)
}
