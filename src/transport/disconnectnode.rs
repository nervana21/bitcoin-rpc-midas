//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde_json::json;
/// Immediately disconnects from the specified peer node.
/// Strictly one out of 'address' and 'nodeid' can be provided to identify the node.
/// To disconnect by nodeid, either set 'address' to the empty string, or call using the named 'nodeid' argument only.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.disconnectnode(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::disconnectnode;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = disconnectnode(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};

/// Calls the `disconnectnode` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn disconnectnode(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    nodeid: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(address), json!(nodeid)];
    let raw = transport.send_request("disconnectnode", &params).await?;
    Ok(raw)
}
