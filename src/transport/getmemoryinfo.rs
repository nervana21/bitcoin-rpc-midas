//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns an object containing information about memory usage.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getmemoryinfo(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getmemoryinfo;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getmemoryinfo(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns an object containing information about memory usage.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetmemoryinfoResponse {
    Variant1(serde_json::Value),
    Variant2(String),
}

/// Calls the `getmemoryinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmemoryinfo(
    transport: &dyn TransportTrait,
    mode: serde_json::Value,
) -> Result<GetmemoryinfoResponse, TransportError> {
    let params = vec![json!(mode)];
    let raw = transport.send_request("getmemoryinfo", &params).await?;
    Ok(serde_json::from_value::<GetmemoryinfoResponse>(raw)?)
}
