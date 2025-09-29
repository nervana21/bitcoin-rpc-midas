//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Verify a signed message.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.verifymessage(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::verifymessage;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = verifymessage(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Verify a signed message.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VerifymessageResponse(pub bool);

/// Calls the `verifymessage` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn verifymessage(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    signature: serde_json::Value,
    message: serde_json::Value,
) -> Result<VerifymessageResponse, TransportError> {
    let params = vec![json!(address), json!(signature), json!(message)];
    let raw = transport.send_request("verifymessage", &params).await?;
    Ok(serde_json::from_value::<VerifymessageResponse>(raw)?)
}
