//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde_json::json;
/// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.backupwallet(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::backupwallet;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = backupwallet(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};

/// Calls the `backupwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn backupwallet(
    transport: &dyn TransportTrait,
    destination: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(destination)];
    let raw = transport.send_request("backupwallet", &params).await?;
    Ok(raw)
}
