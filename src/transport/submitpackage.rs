//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::submitpackage;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.submitpackage(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `submitpackage` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitpackageResponse {
    /// The transaction package result message. \"success\" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// transaction results keyed by wtxid
    pub tx_results: serde_json::Value,
    /// List of txids of replaced transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_transactions: Option<Vec<serde_json::Value>>,
}



/// Calls the `submitpackage` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn submitpackage(transport: &dyn Transport, package: serde_json::Value, maxfeerate: serde_json::Value, maxburnamount: serde_json::Value) -> Result<SubmitpackageResponse, TransportError> {
    let params = vec![json!(package), json!(maxfeerate), json!(maxburnamount)];
    let raw = transport.send_request("submitpackage", &params).await?;
    Ok(serde_json::from_value::<SubmitpackageResponse>(raw)?)
}
