//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns the status of one or all available indices currently running in the node.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getindexinfo(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getindexinfo;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getindexinfo(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns the status of one or all available indices currently running in the node.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetindexinfoResponse {
    pub name: serde_json::Value,
}

/// Calls the `getindexinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getindexinfo(
    transport: &dyn TransportTrait,
    index_name: serde_json::Value,
) -> Result<GetindexinfoResponse, TransportError> {
    let params = vec![json!(index_name)];
    let raw = transport.send_request("getindexinfo", &params).await?;
    Ok(serde_json::from_value::<GetindexinfoResponse>(raw)?)
}
