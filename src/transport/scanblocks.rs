//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.scanblocks(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::scanblocks;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = scanblocks(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ScanblocksResponse {
    Started {
        from_height: u64,
        to_height: u64,
        relevant_blocks: Vec<serde_json::Value>,
        completed: bool,
    },
    Status {
        progress: u64,
        current_height: u64,
    },
    Aborted(bool),
}

/// Calls the `scanblocks` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn scanblocks(
    transport: &dyn TransportTrait,
    action: serde_json::Value,
    scanobjects: serde_json::Value,
    start_height: serde_json::Value,
    stop_height: serde_json::Value,
    filtertype: serde_json::Value,
    options: serde_json::Value,
) -> Result<ScanblocksResponse, TransportError> {
    let params = vec![
        json!(action),
        json!(scanobjects),
        json!(start_height),
        json!(stop_height),
        json!(filtertype),
        json!(options),
    ];
    let raw = transport.send_request("scanblocks", &params).await?;
    Ok(serde_json::from_value::<ScanblocksResponse>(raw)?)
}
