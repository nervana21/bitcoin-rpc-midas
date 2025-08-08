//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns an object containing various wallet state info.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getwalletinfo;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getwalletinfo().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns an object containing various wallet state info.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetwalletinfoResponse {
    pub walletname: String,
    pub walletversion: u32,
    pub format: String,
    pub txcount: u64,
    pub keypoolsize: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keypoolsize_hd_internal: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlocked_until: Option<serde_json::Value>,
    pub paytxfee: f64,
    pub private_keys_enabled: bool,
    pub avoid_reuse: bool,
    pub scanning: serde_json::Value,
    pub descriptors: bool,
    pub external_signer: bool,
    pub blank: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthtime: Option<serde_json::Value>,
    pub flags: Vec<serde_json::Value>,
    pub lastprocessedblock: serde_json::Value,
}



/// Calls the `getwalletinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getwalletinfo(transport: &dyn TransportTrait) -> Result<GetwalletinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getwalletinfo", &params).await?;
    Ok(serde_json::from_value::<GetwalletinfoResponse>(raw)?)
}
