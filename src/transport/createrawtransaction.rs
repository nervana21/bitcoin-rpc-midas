//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.createrawtransaction(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::createrawtransaction;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = createrawtransaction(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreaterawtransactionResponse(pub String);

/// Calls the `createrawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn createrawtransaction(
    transport: &dyn TransportTrait,
    inputs: serde_json::Value,
    outputs: serde_json::Value,
    locktime: serde_json::Value,
    replaceable: serde_json::Value,
    version: serde_json::Value,
) -> Result<CreaterawtransactionResponse, TransportError> {
    let params =
        vec![json!(inputs), json!(outputs), json!(locktime), json!(replaceable), json!(version)];
    let raw = transport.send_request("createrawtransaction", &params).await?;
    Ok(serde_json::from_value::<CreaterawtransactionResponse>(raw)?)
}
