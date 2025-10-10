//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Verifies blockchain database.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.verifychain(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::verifychain;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = verifychain(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Verifies blockchain database.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VerifychainResponse(pub bool);

/// Calls the `verifychain` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn verifychain(
    transport: &dyn TransportTrait,
    checklevel: serde_json::Value,
    nblocks: serde_json::Value,
) -> Result<VerifychainResponse, TransportError> {
    let params = vec![json!(checklevel), json!(nblocks)];
    let raw = transport.send_request("verifychain", &params).await?;
    Ok(serde_json::from_value::<VerifychainResponse>(raw)?)
}
