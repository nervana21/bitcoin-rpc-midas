//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Combine multiple partially signed Bitcoin transactions into one transaction.
/// Implements the Combiner role.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.combinepsbt(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::combinepsbt;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = combinepsbt(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Combine multiple partially signed Bitcoin transactions into one transaction.
/// Implements the Combiner role.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CombinepsbtResponse(pub String);

/// Calls the `combinepsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn combinepsbt(
    transport: &dyn TransportTrait,
    txs: serde_json::Value,
) -> Result<CombinepsbtResponse, TransportError> {
    let params = vec![json!(txs)];
    let raw = transport.send_request("combinepsbt", &params).await?;
    Ok(serde_json::from_value::<CombinepsbtResponse>(raw)?)
}
