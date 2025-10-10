//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
/// Lists groups of addresses which have had their common ownership
/// made public by common use as inputs or as the resulting change
/// in past transactions
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.listaddressgroupings().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::listaddressgroupings;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = listaddressgroupings(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Lists groups of addresses which have had their common ownership
/// made public by common use as inputs or as the resulting change
/// in past transactions
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListaddressgroupingsResponse(pub Vec<serde_json::Value>);

/// Calls the `listaddressgroupings` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listaddressgroupings(
    transport: &dyn TransportTrait,
) -> Result<ListaddressgroupingsResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listaddressgroupings", &params).await?;
    Ok(serde_json::from_value::<ListaddressgroupingsResponse>(raw)?)
}
