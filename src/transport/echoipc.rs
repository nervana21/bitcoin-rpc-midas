//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Echo back the input argument, passing it through a spawned process in a multiprocess build.
/// This command is for testing.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.echoipc(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::echoipc;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = echoipc(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Echo back the input argument, passing it through a spawned process in a multiprocess build.
/// This command is for testing.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EchoipcResponse(pub String);

/// Calls the `echoipc` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn echoipc(
    transport: &dyn TransportTrait,
    arg: serde_json::Value,
) -> Result<EchoipcResponse, TransportError> {
    let params = vec![json!(arg)];
    let raw = transport.send_request("echoipc", &params).await?;
    Ok(serde_json::from_value::<EchoipcResponse>(raw)?)
}
