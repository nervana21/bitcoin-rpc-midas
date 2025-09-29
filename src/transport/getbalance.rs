//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getbalance(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getbalance;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getbalance(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetbalanceResponse(pub f64);

/// Calls the `getbalance` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getbalance(
    transport: &dyn TransportTrait,
    dummy: serde_json::Value,
    minconf: serde_json::Value,
    include_watchonly: serde_json::Value,
    avoid_reuse: serde_json::Value,
) -> Result<GetbalanceResponse, TransportError> {
    let params = vec![json!(dummy), json!(minconf), json!(include_watchonly), json!(avoid_reuse)];
    let raw = transport.send_request("getbalance", &params).await?;
    Ok(serde_json::from_value::<GetbalanceResponse>(raw)?)
}
