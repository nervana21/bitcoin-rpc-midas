//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
/// Unless the the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::dumptxoutset;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.dumptxoutset(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `dumptxoutset` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DumptxoutsetResponse {
    /// the number of coins written in the snapshot
    pub coins_written: u64,
    /// the hash of the base of the snapshot
    pub base_hash: String,
    /// the height of the base of the snapshot
    pub base_height: u64,
    /// the absolute path that the snapshot was written to
    pub path: String,
    /// the hash of the UTXO set contents
    pub txoutset_hash: String,
    /// the number of transactions in the chain up to and including the base block
    pub nchaintx: u64,
}



/// Calls the `dumptxoutset` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn dumptxoutset(transport: &dyn Transport, path: serde_json::Value, r#type: serde_json::Value, options: serde_json::Value) -> Result<DumptxoutsetResponse, TransportError> {
    let params = vec![json!(path), json!(r#type), json!(options)];
    let raw = transport.send_request("dumptxoutset", &params).await?;
    Ok(serde_json::from_value::<DumptxoutsetResponse>(raw)?)
}
