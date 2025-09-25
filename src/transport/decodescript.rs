//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Decode a hex-encoded script.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.decodescript(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::decodescript;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = decodescript(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Decode a hex-encoded script.
#[derive(Debug, Deserialize, Serialize)]
pub struct DecodescriptResponse {
    pub asm: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2sh: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segwit: Option<serde_json::Value>,
}

/// Calls the `decodescript` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn decodescript(
    transport: &dyn TransportTrait,
    hexstring: serde_json::Value,
) -> Result<DecodescriptResponse, TransportError> {
    let params = vec![json!(hexstring)];
    let raw = transport.send_request("decodescript", &params).await?;
    Ok(serde_json::from_value::<DecodescriptResponse>(raw)?)
}
