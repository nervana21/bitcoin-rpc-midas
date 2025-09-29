//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Return information about the given bitcoin address.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.validateaddress(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::validateaddress;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = validateaddress(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Return information about the given bitcoin address.
#[derive(Debug, Deserialize, Serialize)]
pub struct ValidateaddressResponse {
    pub isvalid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "scriptPubKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_pubkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isscript: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iswitness: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_locations: Option<Vec<serde_json::Value>>,
}

/// Calls the `validateaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn validateaddress(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
) -> Result<ValidateaddressResponse, TransportError> {
    let params = vec![json!(address)];
    let raw = transport.send_request("validateaddress", &params).await?;
    Ok(serde_json::from_value::<ValidateaddressResponse>(raw)?)
}
