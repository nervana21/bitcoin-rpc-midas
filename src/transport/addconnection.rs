//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Open an outbound connection to a specified node. This RPC is for testing only.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.addconnection(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::addconnection;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = addconnection(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Open an outbound connection to a specified node. This RPC is for testing only.
#[derive(Debug, Deserialize, Serialize)]
pub struct AddconnectionResponse {
    pub address: String,
    pub connection_type: String,
}

/// Calls the `addconnection` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn addconnection(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    connection_type: serde_json::Value,
    v2transport: serde_json::Value,
) -> Result<AddconnectionResponse, TransportError> {
    let params = vec![json!(address), json!(connection_type), json!(v2transport)];
    let raw = transport.send_request("addconnection", &params).await?;
    Ok(serde_json::from_value::<AddconnectionResponse>(raw)?)
}
