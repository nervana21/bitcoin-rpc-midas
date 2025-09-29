//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Waits for the validation interface queue to catch up on everything that was there when we entered this function.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.syncwithvalidationinterfacequeue().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::syncwithvalidationinterfacequeue;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = syncwithvalidationinterfacequeue(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};

/// Calls the `syncwithvalidationinterfacequeue` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn syncwithvalidationinterfacequeue(
    transport: &dyn TransportTrait,
) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("syncwithvalidationinterfacequeue", &params).await?;
    Ok(raw)
}
