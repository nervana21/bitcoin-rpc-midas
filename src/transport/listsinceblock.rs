//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
/// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.listsinceblock(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::listsinceblock;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = listsinceblock(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
/// If \"blockhash\" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the \"removed\" array.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListsinceblockResponse {
    pub transactions: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<serde_json::Value>>,
    pub lastblock: String,
}

/// Calls the `listsinceblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listsinceblock(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    target_confirmations: serde_json::Value,
    include_watchonly: serde_json::Value,
    include_removed: serde_json::Value,
    include_change: serde_json::Value,
    label: serde_json::Value,
) -> Result<ListsinceblockResponse, TransportError> {
    let params = vec![
        json!(blockhash),
        json!(target_confirmations),
        json!(include_watchonly),
        json!(include_removed),
        json!(include_change),
        json!(label),
    ];
    let raw = transport.send_request("listsinceblock", &params).await?;
    Ok(serde_json::from_value::<ListsinceblockResponse>(raw)?)
}
