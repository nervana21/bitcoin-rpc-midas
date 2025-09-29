//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.verifytxoutproof(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::verifytxoutproof;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = verifytxoutproof(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VerifytxoutproofResponse(pub Vec<serde_json::Value>);

/// Calls the `verifytxoutproof` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn verifytxoutproof(
    transport: &dyn TransportTrait,
    proof: serde_json::Value,
) -> Result<VerifytxoutproofResponse, TransportError> {
    let params = vec![json!(proof)];
    let raw = transport.send_request("verifytxoutproof", &params).await?;
    Ok(serde_json::from_value::<VerifytxoutproofResponse>(raw)?)
}
