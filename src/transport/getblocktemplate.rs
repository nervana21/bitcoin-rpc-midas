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
use transport::{Transport, TransportError};
/// Response for the `getblocktemplate` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblocktemplateResponse {
    /// According to BIP22
    pub field_1: String,
    pub field_2: serde_json::Value,
}



/// Calls the `getblocktemplate` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblocktemplate(transport: &dyn Transport, template_request: serde_json::Value) -> Result<GetblocktemplateResponse, TransportError> {
    let params = vec![json!(template_request)];
    let raw = transport.send_request("getblocktemplate", &params).await?;
    Ok(serde_json::from_value::<GetblocktemplateResponse>(raw)?)
}
