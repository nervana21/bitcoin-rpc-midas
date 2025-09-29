//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getaddednodeinfo(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getaddednodeinfo;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getaddednodeinfo(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetaddednodeinfoResponse(pub Vec<serde_json::Value>);

/// Calls the `getaddednodeinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getaddednodeinfo(
    transport: &dyn TransportTrait,
    node: serde_json::Value,
) -> Result<GetaddednodeinfoResponse, TransportError> {
    let params = vec![json!(node)];
    let raw = transport.send_request("getaddednodeinfo", &params).await?;
    Ok(serde_json::from_value::<GetaddednodeinfoResponse>(raw)?)
}
