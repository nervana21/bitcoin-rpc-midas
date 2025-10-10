//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// List all BIP 32 HD keys in the wallet and which descriptors use them.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.gethdkeys(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::gethdkeys;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = gethdkeys(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// List all BIP 32 HD keys in the wallet and which descriptors use them.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GethdkeysResponse(pub Vec<serde_json::Value>);

/// Calls the `gethdkeys` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gethdkeys(
    transport: &dyn TransportTrait,
    options: serde_json::Value,
) -> Result<GethdkeysResponse, TransportError> {
    let params = vec![json!(options)];
    let raw = transport.send_request("gethdkeys", &params).await?;
    Ok(serde_json::from_value::<GethdkeysResponse>(raw)?)
}
