//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
/// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::dumptxoutset;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.dumptxoutset(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
///
/// Unless the \"latest\" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
///
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
pub struct DumptxoutsetResponse {
    pub coins_written: u64,
    pub base_hash: String,
    pub base_height: u64,
    pub path: String,
    pub txoutset_hash: String,
    pub nchaintx: u64,
}

/// Calls the `dumptxoutset` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn dumptxoutset(
    transport: &dyn TransportTrait,
    path: serde_json::Value,
    r#type: serde_json::Value,
    options: serde_json::Value,
) -> Result<DumptxoutsetResponse, TransportError> {
    let params = vec![json!(path), json!(r#type), json!(options)];
    let raw = transport.send_request("dumptxoutset", &params).await?;
    Ok(serde_json::from_value::<DumptxoutsetResponse>(raw)?)
}
