//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
/// Returns a list of wallets in the wallet directory.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.listwalletdir().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::listwalletdir;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = listwalletdir(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns a list of wallets in the wallet directory.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListwalletdirResponse {
    pub wallets: Vec<String>,
}

/// Calls the `listwalletdir` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listwalletdir(
    transport: &dyn TransportTrait,
) -> Result<ListwalletdirResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listwalletdir", &params).await?;
    Ok(serde_json::from_value::<ListwalletdirResponse>(raw)?)
}
