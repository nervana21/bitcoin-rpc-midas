//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde_json::json;
/// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.importprunedfunds(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::importprunedfunds;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = importprunedfunds(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};

/// Calls the `importprunedfunds` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn importprunedfunds(
    transport: &dyn TransportTrait,
    rawtransaction: serde_json::Value,
    txoutproof: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(rawtransaction), json!(txoutproof)];
    let raw = transport.send_request("importprunedfunds", &params).await?;
    Ok(raw)
}
