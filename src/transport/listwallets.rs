//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use "getwalletinfo"
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.listwallets().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::listwallets;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = listwallets(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use \"getwalletinfo\"
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListwalletsResponse(pub Vec<serde_json::Value>);

/// Calls the `listwallets` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listwallets(
    transport: &dyn TransportTrait,
) -> Result<ListwalletsResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listwallets", &params).await?;
    Ok(serde_json::from_value::<ListwalletsResponse>(raw)?)
}
