//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
/// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::loadtxoutset;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.loadtxoutset(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `loadtxoutset` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LoadtxoutsetResponse {
    /// the number of coins loaded from the snapshot
    pub coins_loaded: u64,
    /// the hash of the base of the snapshot
    pub tip_hash: String,
    /// the height of the base of the snapshot
    pub base_height: u64,
    /// the absolute path that the snapshot was loaded from
    pub path: String,
}



/// Calls the `loadtxoutset` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn loadtxoutset(transport: &dyn Transport, path: serde_json::Value) -> Result<LoadtxoutsetResponse, TransportError> {
    let params = vec![json!(path)];
    let raw = transport.send_request("loadtxoutset", &params).await?;
    Ok(serde_json::from_value::<LoadtxoutsetResponse>(raw)?)
}
