//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getnodeaddresses;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getnodeaddresses(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetnodeaddressesResponse(pub Vec<serde_json::Value>);

/// Calls the `getnodeaddresses` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getnodeaddresses(
    transport: &dyn TransportTrait,
    count: serde_json::Value,
    network: serde_json::Value,
) -> Result<GetnodeaddressesResponse, TransportError> {
    let params = vec![json!(count), json!(network)];
    let raw = transport.send_request("getnodeaddresses", &params).await?;
    Ok(serde_json::from_value::<GetnodeaddressesResponse>(raw)?)
}
