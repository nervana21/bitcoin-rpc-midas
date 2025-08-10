//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Returns mempool data for given transaction

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::getmempoolentry;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getmempoolentry(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns mempool data for given transaction
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmempoolentryResponse {
    pub vsize: u64,
    pub weight: u64,
    pub time: serde_json::Value,
    pub height: u64,
    pub descendantcount: u64,
    pub descendantsize: u64,
    pub ancestorcount: u64,
    pub ancestorsize: u64,
    pub wtxid: bitcoin::Txid,
    pub fees: serde_json::Value,
    pub depends: Vec<serde_json::Value>,
    pub spentby: Vec<serde_json::Value>,
    pub bip125_replaceable: bool,
    pub unbroadcast: bool,
}

/// Calls the `getmempoolentry` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmempoolentry(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
) -> Result<GetmempoolentryResponse, TransportError> {
    let params = vec![json!(txid)];
    let raw = transport.send_request("getmempoolentry", &params).await?;
    Ok(serde_json::from_value::<GetmempoolentryResponse>(raw)?)
}
