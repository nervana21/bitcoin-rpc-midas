//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// List balances by receiving address.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.listreceivedbyaddress(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::listreceivedbyaddress;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = listreceivedbyaddress(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// List balances by receiving address.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListreceivedbyaddressResponse(pub Vec<serde_json::Value>);

/// Calls the `listreceivedbyaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listreceivedbyaddress(
    transport: &dyn TransportTrait,
    minconf: serde_json::Value,
    include_empty: serde_json::Value,
    include_watchonly: serde_json::Value,
    address_filter: serde_json::Value,
    include_immature_coinbase: serde_json::Value,
) -> Result<ListreceivedbyaddressResponse, TransportError> {
    let params = vec![
        json!(minconf),
        json!(include_empty),
        json!(include_watchonly),
        json!(address_filter),
        json!(include_immature_coinbase),
    ];
    let raw = transport.send_request("listreceivedbyaddress", &params).await?;
    Ok(serde_json::from_value::<ListreceivedbyaddressResponse>(raw)?)
}
