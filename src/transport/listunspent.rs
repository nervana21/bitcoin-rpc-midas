//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.listunspent(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::listunspent;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = listunspent(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListunspentResponse(pub Vec<serde_json::Value>);

/// Calls the `listunspent` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listunspent(
    transport: &dyn TransportTrait,
    minconf: serde_json::Value,
    maxconf: serde_json::Value,
    addresses: serde_json::Value,
    include_unsafe: serde_json::Value,
    query_options: serde_json::Value,
) -> Result<ListunspentResponse, TransportError> {
    let params = vec![
        json!(minconf),
        json!(maxconf),
        json!(addresses),
        json!(include_unsafe),
        json!(query_options),
    ];
    let raw = transport.send_request("listunspent", &params).await?;
    Ok(serde_json::from_value::<ListunspentResponse>(raw)?)
}
