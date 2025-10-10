//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns the total amount received by the given address in transactions with at least minconf confirmations.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getreceivedbyaddress(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getreceivedbyaddress;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getreceivedbyaddress(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns the total amount received by the given address in transactions with at least minconf confirmations.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetreceivedbyaddressResponse(pub f64);

/// Calls the `getreceivedbyaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getreceivedbyaddress(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    minconf: serde_json::Value,
    include_immature_coinbase: serde_json::Value,
) -> Result<GetreceivedbyaddressResponse, TransportError> {
    let params = vec![json!(address), json!(minconf), json!(include_immature_coinbase)];
    let raw = transport.send_request("getreceivedbyaddress", &params).await?;
    Ok(serde_json::from_value::<GetreceivedbyaddressResponse>(raw)?)
}
