//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
/// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
/// Note: Use "getwalletinfo" to query the scanning progress.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.abortrescan().await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::abortrescan;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = abortrescan(&transport).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
/// Note: Use \"getwalletinfo\" to query the scanning progress.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AbortrescanResponse(pub bool);

/// Calls the `abortrescan` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn abortrescan(
    transport: &dyn TransportTrait,
) -> Result<AbortrescanResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("abortrescan", &params).await?;
    Ok(serde_json::from_value::<AbortrescanResponse>(raw)?)
}
