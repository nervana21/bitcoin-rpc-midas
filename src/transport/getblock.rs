//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getblock(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getblock;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getblock(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GetblockResponse {
    Raw(String),
    Verbose {
        hash: String,
        confirmations: u64,
        size: u64,
        strippedsize: u64,
        weight: u64,
        height: u64,
        version: u32,
        #[serde(rename = "versionHex")]
        version_hex: String,
        merkleroot: String,
        tx: Vec<serde_json::Value>,
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
    Detailed {
        field_0: serde_json::Value,
        tx: Vec<serde_json::Value>,
    },
    Full {
        field_0: serde_json::Value,
        tx: Vec<serde_json::Value>,
    },
}

/// Calls the `getblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblock(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    verbosity: serde_json::Value,
) -> Result<GetblockResponse, TransportError> {
    let params = vec![json!(blockhash), json!(verbosity)];
    let raw = transport.send_request("getblock", &params).await?;
    Ok(serde_json::from_value::<GetblockResponse>(raw)?)
}
