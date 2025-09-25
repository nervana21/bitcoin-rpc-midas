//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
/// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.dumptxoutset(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::dumptxoutset;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = dumptxoutset(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
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
