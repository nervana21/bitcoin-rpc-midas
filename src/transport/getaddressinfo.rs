//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getaddressinfo(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getaddressinfo;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getaddressinfo(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetaddressinfoResponse {
    pub address: String,
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: String,
    pub ismine: bool,
    pub iswatchonly: bool,
    pub solvable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isscript: Option<bool>,
    pub ischange: bool,
    pub iswitness: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkeys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sigsrequired: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscompressed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdkeypath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdseedid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdmasterfingerprint: Option<String>,
    pub labels: Vec<serde_json::Value>,
}

/// Calls the `getaddressinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getaddressinfo(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
) -> Result<GetaddressinfoResponse, TransportError> {
    let params = vec![json!(address)];
    let raw = transport.send_request("getaddressinfo", &params).await?;
    Ok(serde_json::from_value::<GetaddressinfoResponse>(raw)?)
}
