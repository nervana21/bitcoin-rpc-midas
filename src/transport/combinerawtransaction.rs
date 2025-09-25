//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.combinerawtransaction(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::combinerawtransaction;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = combinerawtransaction(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CombinerawtransactionResponse(pub String);

/// Calls the `combinerawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn combinerawtransaction(
    transport: &dyn TransportTrait,
    txs: serde_json::Value,
) -> Result<CombinerawtransactionResponse, TransportError> {
    let params = vec![json!(txs)];
    let raw = transport.send_request("combinerawtransaction", &params).await?;
    Ok(serde_json::from_value::<CombinerawtransactionResponse>(raw)?)
}
