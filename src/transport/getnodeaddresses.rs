//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getnodeaddresses;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getnodeaddresses(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getnodeaddresses` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnodeaddressesResponse {
    pub field_0: Vec<serde_json::Value>,
}



/// Calls the `getnodeaddresses` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getnodeaddresses(transport: &dyn Transport, count: serde_json::Value, network: serde_json::Value) -> Result<GetnodeaddressesResponse, TransportError> {
    let params = vec![json!(count), json!(network)];
    let raw = transport.send_request("getnodeaddresses", &params).await?;
    Ok(serde_json::from_value::<GetnodeaddressesResponse>(raw)?)
}
