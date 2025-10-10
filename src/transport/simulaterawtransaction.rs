//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.simulaterawtransaction(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::simulaterawtransaction;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = simulaterawtransaction(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
#[derive(Debug, Deserialize, Serialize)]
pub struct SimulaterawtransactionResponse {
    pub balance_change: f64,
}

/// Calls the `simulaterawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn simulaterawtransaction(
    transport: &dyn TransportTrait,
    rawtxs: serde_json::Value,
    options: serde_json::Value,
) -> Result<SimulaterawtransactionResponse, TransportError> {
    let params = vec![json!(rawtxs), json!(options)];
    let raw = transport.send_request("simulaterawtransaction", &params).await?;
    Ok(serde_json::from_value::<SimulaterawtransactionResponse>(raw)?)
}
