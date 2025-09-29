//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.generateblock(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::generateblock;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = generateblock(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateblockResponse {
    pub hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

/// Calls the `generateblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn generateblock(
    transport: &dyn TransportTrait,
    output: serde_json::Value,
    transactions: serde_json::Value,
    submit: serde_json::Value,
) -> Result<GenerateblockResponse, TransportError> {
    let params = vec![json!(output), json!(transactions), json!(submit)];
    let raw = transport.send_request("generateblock", &params).await?;
    Ok(serde_json::from_value::<GenerateblockResponse>(raw)?)
}
