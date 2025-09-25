//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns an object containing various state info regarding deployments of consensus changes.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getdeploymentinfo(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getdeploymentinfo;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getdeploymentinfo(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns an object containing various state info regarding deployments of consensus changes.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetdeploymentinfoResponse {
    pub hash: String,
    pub height: u64,
    pub deployments: serde_json::Value,
}

/// Calls the `getdeploymentinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getdeploymentinfo(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
) -> Result<GetdeploymentinfoResponse, TransportError> {
    let params = vec![json!(blockhash)];
    let raw = transport.send_request("getdeploymentinfo", &params).await?;
    Ok(serde_json::from_value::<GetdeploymentinfoResponse>(raw)?)
}
