//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::submitpackage;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.submitpackage(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
#[derive(Debug, Deserialize, Serialize)]
pub struct SubmitpackageResponse {
    pub package_msg: String,
    pub tx_results: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_transactions: Option<Vec<serde_json::Value>>,
}

/// Calls the `submitpackage` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn submitpackage(
    transport: &dyn TransportTrait,
    package: serde_json::Value,
    maxfeerate: serde_json::Value,
    maxburnamount: serde_json::Value,
) -> Result<SubmitpackageResponse, TransportError> {
    let params = vec![json!(package), json!(maxfeerate), json!(maxburnamount)];
    let raw = transport.send_request("submitpackage", &params).await?;
    Ok(serde_json::from_value::<SubmitpackageResponse>(raw)?)
}
