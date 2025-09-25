//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getnetworkhashps(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getnetworkhashps;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getnetworkhashps(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetnetworkhashpsResponse(pub u64);

/// Calls the `getnetworkhashps` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getnetworkhashps(
    transport: &dyn TransportTrait,
    nblocks: serde_json::Value,
    height: serde_json::Value,
) -> Result<GetnetworkhashpsResponse, TransportError> {
    let params = vec![json!(nblocks), json!(height)];
    let raw = transport.send_request("getnetworkhashps", &params).await?;
    Ok(serde_json::from_value::<GetnetworkhashpsResponse>(raw)?)
}
