//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.sendmsgtopeer(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::sendmsgtopeer;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = sendmsgtopeer(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SendmsgtopeerResponse(pub serde_json::Value);

/// Calls the `sendmsgtopeer` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn sendmsgtopeer(
    transport: &dyn TransportTrait,
    peer_id: serde_json::Value,
    msg_type: serde_json::Value,
    msg: serde_json::Value,
) -> Result<SendmsgtopeerResponse, TransportError> {
    let params = vec![json!(peer_id), json!(msg_type), json!(msg)];
    let raw = transport.send_request("sendmsgtopeer", &params).await?;
    Ok(serde_json::from_value::<SendmsgtopeerResponse>(raw)?)
}
