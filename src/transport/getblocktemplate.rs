//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
/// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
/// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getblocktemplate(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getblocktemplate;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getblocktemplate(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
/// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
/// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetblocktemplateResponse {
    Accepted(String),
    Variant3 {
        version: u32,
        rules: Vec<serde_json::Value>,
        vbavailable: serde_json::Value,
        capabilities: Vec<serde_json::Value>,
        vbrequired: u64,
        previousblockhash: bitcoin::BlockHash,
        transactions: Vec<serde_json::Value>,
        coinbaseaux: serde_json::Value,
        coinbasevalue: u64,
        longpollid: String,
        target: String,
        mintime: serde_json::Value,
        mutable: Vec<serde_json::Value>,
        noncerange: String,
        sigoplimit: u64,
        sizelimit: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        weightlimit: Option<u64>,
        curtime: serde_json::Value,
        bits: String,
        height: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        signet_challenge: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        default_witness_commitment: Option<String>,
    },
}

/// Calls the `getblocktemplate` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblocktemplate(
    transport: &dyn TransportTrait,
    template_request: serde_json::Value,
) -> Result<GetblocktemplateResponse, TransportError> {
    let params = vec![json!(template_request)];
    let raw = transport.send_request("getblocktemplate", &params).await?;
    Ok(serde_json::from_value::<GetblocktemplateResponse>(raw)?)
}
