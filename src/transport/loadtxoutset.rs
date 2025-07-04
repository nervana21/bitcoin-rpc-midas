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
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.loadtxoutset(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Load the serialized UTXO set from a file.
    /// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
    /// 
    /// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
    /// 
    /// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
#[derive(Debug, Deserialize, Serialize)]
pub struct LoadtxoutsetResponse {
    pub coins_loaded: u64,
    pub tip_hash: String,
    pub base_height: u64,
    pub path: String,
}



/// Calls the `loadtxoutset` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn loadtxoutset(transport: &dyn TransportTrait, path: serde_json::Value) -> Result<LoadtxoutsetResponse, TransportError> {
    let params = vec![json!(path)];
    let raw = transport.send_request("loadtxoutset", &params).await?;
    Ok(serde_json::from_value::<LoadtxoutsetResponse>(raw)?)
}
