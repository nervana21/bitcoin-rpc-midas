//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// Hint: Use gettransaction for wallet transactions.
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getrawtransaction(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getrawtransaction;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getrawtransaction(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
///
/// Hint: Use gettransaction for wallet transactions.
///
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetrawtransactionResponse {
    Raw(String),
    Verbose {
        #[serde(skip_serializing_if = "Option::is_none")]
        in_active_chain: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        blockhash: Option<bitcoin::BlockHash>,
        #[serde(skip_serializing_if = "Option::is_none")]
        confirmations: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        blocktime: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        time: Option<u64>,
        hex: String,
        txid: bitcoin::Txid,
        hash: String,
        size: u64,
        vsize: u64,
        weight: u64,
        version: u32,
        locktime: serde_json::Value,
        vin: Vec<serde_json::Value>,
        vout: Vec<serde_json::Value>,
    },
    Detailed {
        field_0: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        fee: Option<f64>,
        vin: Vec<serde_json::Value>,
    },
}

/// Calls the `getrawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrawtransaction(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    verbosity: serde_json::Value,
    blockhash: serde_json::Value,
) -> Result<GetrawtransactionResponse, TransportError> {
    let params = vec![json!(txid), json!(verbosity), json!(blockhash)];
    let raw = transport.send_request("getrawtransaction", &params).await?;
    Ok(serde_json::from_value::<GetrawtransactionResponse>(raw)?)
}
