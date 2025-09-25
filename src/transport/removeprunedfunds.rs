//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde_json::json;
/// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.removeprunedfunds(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::removeprunedfunds;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = removeprunedfunds(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};

/// Calls the `removeprunedfunds` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn removeprunedfunds(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid)];
    let raw = transport.send_request("removeprunedfunds", &params).await?;
    Ok(raw)
}
