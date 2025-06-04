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
use transport::{Transport, TransportError};
/// Response for the `getmempoolinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmempoolinfoResponse {
    /// True if the initial load attempt of the persisted mempool finished
    pub loaded: bool,
    /// Current tx count
    pub size: u64,
    /// Sum of all virtual transaction sizes as defined in BIP 141. Differs from actual serialized size because witness data is discounted
    pub bytes: u64,
    /// Total memory usage for the mempool
    pub usage: u64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction
    pub total_fee: bitcoin::Amount,
    /// Maximum memory usage for the mempool
    pub maxmempool: u64,
    /// Minimum fee rate in BTC/kvB for tx to be accepted. Is the maximum of minrelaytxfee and minimum mempool fee
    pub mempoolminfee: bitcoin::Amount,
    /// Current minimum relay fee for transactions
    pub minrelaytxfee: bitcoin::Amount,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    pub incrementalrelayfee: u64,
    /// Current number of transactions that haven't passed initial broadcast yet
    pub unbroadcastcount: u64,
    /// True if the mempool accepts RBF without replaceability signaling inspection (DEPRECATED)
    pub fullrbf: bool,
}



/// Calls the `getmempoolinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmempoolinfo(transport: &dyn Transport) -> Result<GetmempoolinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getmempoolinfo", &params).await?;
    Ok(serde_json::from_value::<GetmempoolinfoResponse>(raw)?)
}
