//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Send a transaction.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.send(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::send;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = send(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Send a transaction.
#[derive(Debug, Deserialize, Serialize)]
pub struct SendResponse {
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
}

/// Calls the `send` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn send(
    transport: &dyn TransportTrait,
    outputs: serde_json::Value,
    conf_target: serde_json::Value,
    estimate_mode: serde_json::Value,
    fee_rate: serde_json::Value,
    options: serde_json::Value,
    version: serde_json::Value,
) -> Result<SendResponse, TransportError> {
    let params = vec![
        json!(outputs),
        json!(conf_target),
        json!(estimate_mode),
        json!(fee_rate),
        json!(options),
        json!(version),
    ];
    let raw = transport.send_request("send", &params).await?;
    Ok(serde_json::from_value::<SendResponse>(raw)?)
}
