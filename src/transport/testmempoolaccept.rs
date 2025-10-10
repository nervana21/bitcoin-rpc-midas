//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
/// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
/// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
/// The maximum number of transactions allowed is 25.
/// This checks if transactions violate the consensus or policy rules.
/// See sendrawtransaction call.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.testmempoolaccept(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::testmempoolaccept;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = testmempoolaccept(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
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
pub async fn testmempoolaccept(
    transport: &dyn TransportTrait,
    rawtxs: serde_json::Value,
    maxfeerate: serde_json::Value,
) -> Result<TestmempoolacceptResponse, TransportError> {
    let params = vec![json!(rawtxs), json!(maxfeerate)];
    let raw = transport.send_request("testmempoolaccept", &params).await?;
    Ok(serde_json::from_value::<TestmempoolacceptResponse>(raw)?)
}
