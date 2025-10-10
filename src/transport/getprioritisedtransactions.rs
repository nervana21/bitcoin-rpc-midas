//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
/// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getprioritisedtransactions().await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getprioritisedtransactions;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getprioritisedtransactions(&transport).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetprioritisedtransactionsResponse {
    pub transactionid: serde_json::Value,
}

/// Calls the `getprioritisedtransactions` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getprioritisedtransactions(
    transport: &dyn TransportTrait,
) -> Result<GetprioritisedtransactionsResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getprioritisedtransactions", &params).await?;
    Ok(serde_json::from_value::<GetprioritisedtransactionsResponse>(raw)?)
}
