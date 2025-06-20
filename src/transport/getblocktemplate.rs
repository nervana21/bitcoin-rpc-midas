//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
/// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
/// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getblocktemplate;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getblocktemplate(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
    /// It returns data needed to construct a block to work on.
    /// For full specification, see BIPs 22, 23, 9, and 145:
    /// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
    /// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
    /// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
    /// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
#[derive(Debug, Deserialize, Serialize)]
pub struct GetblocktemplateResponse {
        #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub vbavailable: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub vbrequired: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub previousblockhash: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbaseaux: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbasevalue: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub longpollid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub mintime: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub mutable: Option<Vec<serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub noncerange: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub sigoplimit: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub sizelimit: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub weightlimit: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub curtime: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub bits: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub signet_challenge: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
    pub default_witness_commitment: Option<String>,
}



/// Calls the `getblocktemplate` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblocktemplate(transport: &dyn TransportTrait, template_request: serde_json::Value) -> Result<GetblocktemplateResponse, TransportError> {
    let params = vec![json!(template_request)];
    let raw = transport.send_request("getblocktemplate", &params).await?;
    Ok(serde_json::from_value::<GetblocktemplateResponse>(raw)?)
}
