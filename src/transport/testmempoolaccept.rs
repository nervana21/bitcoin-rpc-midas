//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
/// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
/// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
/// The maximum number of transactions allowed is 25.
/// This checks if transactions violate the consensus or policy rules.
/// See sendrawtransaction call.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::testmempoolaccept;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.testmempoolaccept(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
    /// 
    /// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
    /// 
    /// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
    /// 
    /// The maximum number of transactions allowed is 25.
    /// 
    /// This checks if transactions violate the consensus or policy rules.
    /// 
    /// See sendrawtransaction call.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TestmempoolacceptResponse(pub Vec<serde_json::Value>);



/// Calls the `testmempoolaccept` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn testmempoolaccept(transport: &dyn TransportTrait, rawtxs: serde_json::Value, maxfeerate: serde_json::Value) -> Result<TestmempoolacceptResponse, TransportError> {
    let params = vec![json!(rawtxs), json!(maxfeerate)];
    let raw = transport.send_request("testmempoolaccept", &params).await?;
    Ok(serde_json::from_value::<TestmempoolacceptResponse>(raw)?)
}
