//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getrawchangeaddress(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getrawchangeaddress;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getrawchangeaddress(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetrawchangeaddressResponse(pub String);

/// Calls the `getrawchangeaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrawchangeaddress(
    transport: &dyn TransportTrait,
    address_type: serde_json::Value,
) -> Result<GetrawchangeaddressResponse, TransportError> {
    let params = vec![json!(address_type)];
    let raw = transport.send_request("getrawchangeaddress", &params).await?;
    Ok(serde_json::from_value::<GetrawchangeaddressResponse>(raw)?)
}
