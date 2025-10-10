//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// If txid is in the mempool, returns all in-mempool descendants.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getmempooldescendants(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getmempooldescendants;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getmempooldescendants(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// If txid is in the mempool, returns all in-mempool descendants.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetmempooldescendantsResponse {
    Raw(Vec<String>),
    Verbose(serde_json::Value),
}

/// Calls the `getmempooldescendants` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmempooldescendants(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<GetmempooldescendantsResponse, TransportError> {
    let params = vec![json!(txid), json!(verbose)];
    let raw = transport.send_request("getmempooldescendants", &params).await?;
    Ok(serde_json::from_value::<GetmempooldescendantsResponse>(raw)?)
}
