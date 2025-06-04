//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns mempool data for given transaction

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getmempoolentry;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getmempoolentry(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getmempoolentry` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmempoolentryResponse {
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: u64,
    /// transaction weight as defined in BIP 141.
    pub weight: u64,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: serde_json::Value,
    /// block height when transaction entered pool
    pub height: u64,
    /// number of in-mempool descendant transactions (including this one)
    pub descendantcount: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    pub descendantsize: u64,
    /// number of in-mempool ancestor transactions (including this one)
    pub ancestorcount: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    pub ancestorsize: u64,
    /// hash of serialized transaction, including witness data
    pub wtxid: bitcoin::Txid,
    pub fees: serde_json::Value,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<serde_json::Value>,
    /// unconfirmed transactions spending outputs from this transaction
    pub spentby: Vec<serde_json::Value>,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
    pub bip125_replaceable: bool,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
}



/// Calls the `getmempoolentry` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmempoolentry(transport: &dyn Transport, txid: serde_json::Value) -> Result<GetmempoolentryResponse, TransportError> {
    let params = vec![json!(txid)];
    let raw = transport.send_request("getmempoolentry", &params).await?;
    Ok(serde_json::from_value::<GetmempoolentryResponse>(raw)?)
}
