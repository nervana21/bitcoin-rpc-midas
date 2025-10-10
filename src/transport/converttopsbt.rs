//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// createpsbt and walletcreatefundedpsbt should be used for new applications.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.converttopsbt(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::converttopsbt;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = converttopsbt(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// createpsbt and walletcreatefundedpsbt should be used for new applications.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConverttopsbtResponse(pub String);

/// Calls the `converttopsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn converttopsbt(
    transport: &dyn TransportTrait,
    hexstring: serde_json::Value,
    permitsigdata: serde_json::Value,
    iswitness: serde_json::Value,
) -> Result<ConverttopsbtResponse, TransportError> {
    let params = vec![json!(hexstring), json!(permitsigdata), json!(iswitness)];
    let raw = transport.send_request("converttopsbt", &params).await?;
    Ok(serde_json::from_value::<ConverttopsbtResponse>(raw)?)
}
