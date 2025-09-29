//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Mine to a specified descriptor and return the block hashes.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.generatetodescriptor(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::generatetodescriptor;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = generatetodescriptor(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Mine to a specified descriptor and return the block hashes.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GeneratetodescriptorResponse(pub Vec<serde_json::Value>);

/// Calls the `generatetodescriptor` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn generatetodescriptor(
    transport: &dyn TransportTrait,
    num_blocks: serde_json::Value,
    descriptor: serde_json::Value,
    maxtries: serde_json::Value,
) -> Result<GeneratetodescriptorResponse, TransportError> {
    let params = vec![json!(num_blocks), json!(descriptor), json!(maxtries)];
    let raw = transport.send_request("generatetodescriptor", &params).await?;
    Ok(serde_json::from_value::<GeneratetodescriptorResponse>(raw)?)
}
