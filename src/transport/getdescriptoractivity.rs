//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
/// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::getdescriptoractivity;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getdescriptoractivity(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
/// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
pub struct GetdescriptoractivityResponse {
    pub activity: Vec<serde_json::Value>,
}

/// Calls the `getdescriptoractivity` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getdescriptoractivity(
    transport: &dyn TransportTrait,
    blockhashes: serde_json::Value,
    scanobjects: serde_json::Value,
    include_mempool: serde_json::Value,
) -> Result<GetdescriptoractivityResponse, TransportError> {
    let params = vec![
        json!(blockhashes),
        json!(scanobjects),
        json!(include_mempool),
    ];
    let raw = transport
        .send_request("getdescriptoractivity", &params)
        .await?;
    Ok(serde_json::from_value::<GetdescriptoractivityResponse>(
        raw,
    )?)
}
