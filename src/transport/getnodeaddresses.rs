//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getnodeaddresses(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getnodeaddresses;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getnodeaddresses(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
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
