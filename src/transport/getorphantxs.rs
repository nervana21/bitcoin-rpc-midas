//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Shows transactions in the tx orphanage.
/// EXPERIMENTAL warning: this call may be changed in future releases.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getorphantxs(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getorphantxs;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getorphantxs(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Shows transactions in the tx orphanage.
///
/// EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetorphantxsResponse {
    Variant1(Vec<bitcoin::Txid>),
    Variant2(Vec<serde_json::Value>),
    Variant3(Vec<serde_json::Value>),
}

/// Calls the `getorphantxs` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getorphantxs(
    transport: &dyn TransportTrait,
    verbosity: serde_json::Value,
) -> Result<GetorphantxsResponse, TransportError> {
    let params = vec![json!(verbosity)];
    let raw = transport.send_request("getorphantxs", &params).await?;
    Ok(serde_json::from_value::<GetorphantxsResponse>(raw)?)
}
