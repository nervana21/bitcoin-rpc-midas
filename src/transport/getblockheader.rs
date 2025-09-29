//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getblockheader(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getblockheader;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getblockheader(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetblockheaderResponse {
    Verbose {
        hash: String,
        confirmations: u64,
        height: u64,
        version: u32,
        #[serde(rename = "versionHex")]
        version_hex: String,
        merkleroot: String,
        time: serde_json::Value,
        mediantime: serde_json::Value,
        nonce: u64,
        bits: String,
        target: String,
        difficulty: f64,
        chainwork: String,
        #[serde(rename = "nTx")]
        n_tx: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        previousblockhash: Option<bitcoin::BlockHash>,
        #[serde(skip_serializing_if = "Option::is_none")]
        nextblockhash: Option<bitcoin::BlockHash>,
    },
    Variant2(String),
}

/// Calls the `getblockheader` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockheader(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<GetblockheaderResponse, TransportError> {
    let params = vec![json!(blockhash), json!(verbose)];
    let raw = transport.send_request("getblockheader", &params).await?;
    Ok(serde_json::from_value::<GetblockheaderResponse>(raw)?)
}
