//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.createpsbt(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::createpsbt;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = createpsbt(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreatepsbtResponse(pub String);

/// Calls the `createpsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn createpsbt(
    transport: &dyn TransportTrait,
    inputs: serde_json::Value,
    outputs: serde_json::Value,
    locktime: serde_json::Value,
    replaceable: serde_json::Value,
    version: serde_json::Value,
) -> Result<CreatepsbtResponse, TransportError> {
    let params =
        vec![json!(inputs), json!(outputs), json!(locktime), json!(replaceable), json!(version)];
    let raw = transport.send_request("createpsbt", &params).await?;
    Ok(serde_json::from_value::<CreatepsbtResponse>(raw)?)
}
