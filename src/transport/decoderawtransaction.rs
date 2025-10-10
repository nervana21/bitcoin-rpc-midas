//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Return a JSON object representing the serialized, hex-encoded transaction.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.decoderawtransaction(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::decoderawtransaction;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = decoderawtransaction(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Return a JSON object representing the serialized, hex-encoded transaction.
#[derive(Debug, Deserialize, Serialize)]
pub struct DecoderawtransactionResponse {
    pub txid: bitcoin::Txid,
    pub hash: String,
    pub size: u64,
    pub vsize: u64,
    pub weight: u64,
    pub version: u32,
    pub locktime: serde_json::Value,
    pub vin: Vec<serde_json::Value>,
    pub vout: Vec<serde_json::Value>,
}

/// Calls the `decoderawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn decoderawtransaction(
    transport: &dyn TransportTrait,
    hexstring: serde_json::Value,
    iswitness: serde_json::Value,
) -> Result<DecoderawtransactionResponse, TransportError> {
    let params = vec![json!(hexstring), json!(iswitness)];
    let raw = transport.send_request("decoderawtransaction", &params).await?;
    Ok(serde_json::from_value::<DecoderawtransactionResponse>(raw)?)
}
