//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Scans the mempool to find transactions spending any of the given outputs
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.gettxspendingprevout(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::gettxspendingprevout;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = gettxspendingprevout(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Scans the mempool to find transactions spending any of the given outputs
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GettxspendingprevoutResponse(pub Vec<serde_json::Value>);

/// Calls the `gettxspendingprevout` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettxspendingprevout(
    transport: &dyn TransportTrait,
    outputs: serde_json::Value,
) -> Result<GettxspendingprevoutResponse, TransportError> {
    let params = vec![json!(outputs)];
    let raw = transport.send_request("gettxspendingprevout", &params).await?;
    Ok(serde_json::from_value::<GettxspendingprevoutResponse>(raw)?)
}
