//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Add the address of a potential peer to an address manager table. This RPC is for testing only.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::addpeeraddress;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.addpeeraddress(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Add the address of a potential peer to an address manager table. This RPC is for testing only.
#[derive(Debug, Deserialize, Serialize)]
pub struct AddpeeraddressResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}



/// Calls the `addpeeraddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn addpeeraddress(transport: &dyn TransportTrait, address: serde_json::Value, port: serde_json::Value, tried: serde_json::Value) -> Result<AddpeeraddressResponse, TransportError> {
    let params = vec![json!(address), json!(port), json!(tried)];
    let raw = transport.send_request("addpeeraddress", &params).await?;
    Ok(serde_json::from_value::<AddpeeraddressResponse>(raw)?)
}
