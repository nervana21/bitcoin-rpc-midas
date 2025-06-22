//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Return information about the given bitcoin address.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::validateaddress;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.validateaddress(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Return information about the given bitcoin address.
#[derive(Debug, Deserialize, Serialize)]
pub struct ValidateaddressResponse {
    pub isvalid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_pub_key: Option<String>,
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
pub async fn validateaddress(transport: &dyn TransportTrait, address: serde_json::Value) -> Result<ValidateaddressResponse, TransportError> {
    let params = vec![json!(address)];
    let raw = transport.send_request("validateaddress", &params).await?;
    Ok(serde_json::from_value::<ValidateaddressResponse>(raw)?)
}
