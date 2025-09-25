//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getrawmempool(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getrawmempool;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getrawmempool(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns all transaction ids in memory pool as a json array of string transaction ids.
///
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GetrawmempoolResponse {
    Raw(Vec<String>),
    Verbose(serde_json::Value),
    RawWithSequence { txids: Vec<bitcoin::Txid>, mempool_sequence: u64 },
}

/// Calls the `getrawmempool` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrawmempool(
    transport: &dyn TransportTrait,
    verbose: serde_json::Value,
    mempool_sequence: serde_json::Value,
) -> Result<GetrawmempoolResponse, TransportError> {
    let params = vec![json!(verbose), json!(mempool_sequence)];
    let raw = transport.send_request("getrawmempool", &params).await?;
    Ok(serde_json::from_value::<GetrawmempoolResponse>(raw)?)
}
