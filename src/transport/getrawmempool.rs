//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getrawmempool;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getrawmempool(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns all transaction ids in memory pool as a json array of string transaction ids.
///
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetrawmempoolResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactionid: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txids: Option<Vec<bitcoin::Txid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mempool_sequence: Option<u64>,
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
