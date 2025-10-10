//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Retrieve a BIP 157 content filter for a particular block.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getblockfilter(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getblockfilter;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getblockfilter(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Retrieve a BIP 157 content filter for a particular block.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetblockfilterResponse {
    pub filter: String,
    pub header: String,
}

/// Calls the `getblockfilter` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockfilter(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    filtertype: serde_json::Value,
) -> Result<GetblockfilterResponse, TransportError> {
    let params = vec![json!(blockhash), json!(filtertype)];
    let raw = transport.send_request("getblockfilter", &params).await?;
    Ok(serde_json::from_value::<GetblockfilterResponse>(raw)?)
}
