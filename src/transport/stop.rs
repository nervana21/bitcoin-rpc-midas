//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Request a graceful shutdown of Bitcoin Core.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.stop(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::stop;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = stop(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Request a graceful shutdown of Bitcoin Core.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StopResponse(pub String);

/// Calls the `stop` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn stop(
    transport: &dyn TransportTrait,
    wait: serde_json::Value,
) -> Result<StopResponse, TransportError> {
    let params = vec![json!(wait)];
    let raw = transport.send_request("stop", &params).await?;
    Ok(serde_json::from_value::<StopResponse>(raw)?)
}
