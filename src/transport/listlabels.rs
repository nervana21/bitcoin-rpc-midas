//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.listlabels(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::listlabels;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = listlabels(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListlabelsResponse(pub Vec<serde_json::Value>);

/// Calls the `listlabels` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listlabels(
    transport: &dyn TransportTrait,
    purpose: serde_json::Value,
) -> Result<ListlabelsResponse, TransportError> {
    let params = vec![json!(purpose)];
    let raw = transport.send_request("listlabels", &params).await?;
    Ok(serde_json::from_value::<ListlabelsResponse>(raw)?)
}
