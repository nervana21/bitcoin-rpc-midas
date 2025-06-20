//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns details on the active state of the TX memory pool.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getmempoolinfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getmempoolinfo().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns details on the active state of the TX memory pool.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmempoolinfoResponse {
    pub loaded: bool,
    pub size: u64,
    pub bytes: f64,
    pub usage: f64,
    pub total_fee: serde_json::Value,
    pub maxmempool: f64,
    pub mempoolminfee: serde_json::Value,
    pub minrelaytxfee: serde_json::Value,
    pub incrementalrelayfee: f64,
    pub unbroadcastcount: u64,
    pub fullrbf: bool,
}



/// Calls the `getmempoolinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmempoolinfo(transport: &dyn TransportTrait) -> Result<GetmempoolinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getmempoolinfo", &params).await?;
    Ok(serde_json::from_value::<GetmempoolinfoResponse>(raw)?)
}
