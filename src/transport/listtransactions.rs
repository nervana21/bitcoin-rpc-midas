//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
/// Returns up to 'count' most recent transactions skipping the first 'from' transactions.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.listtransactions(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::listtransactions;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = listtransactions(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
///
/// Returns up to 'count' most recent transactions skipping the first 'from' transactions.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListtransactionsResponse(pub Vec<serde_json::Value>);

/// Calls the `listtransactions` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listtransactions(
    transport: &dyn TransportTrait,
    label: serde_json::Value,
    count: serde_json::Value,
    skip: serde_json::Value,
    include_watchonly: serde_json::Value,
) -> Result<ListtransactionsResponse, TransportError> {
    let params = vec![json!(label), json!(count), json!(skip), json!(include_watchonly)];
    let raw = transport.send_request("listtransactions", &params).await?;
    Ok(serde_json::from_value::<ListtransactionsResponse>(raw)?)
}
