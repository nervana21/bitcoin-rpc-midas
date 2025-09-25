//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
/// If both are specified, they must be identical.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.unloadwallet(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::unloadwallet;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = unloadwallet(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
/// If both are specified, they must be identical.
#[derive(Debug, Deserialize, Serialize)]
pub struct UnloadwalletResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}

/// Calls the `unloadwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn unloadwallet(
    transport: &dyn TransportTrait,
    wallet_name: serde_json::Value,
    load_on_startup: serde_json::Value,
) -> Result<UnloadwalletResponse, TransportError> {
    let params = vec![json!(wallet_name), json!(load_on_startup)];
    let raw = transport.send_request("unloadwallet", &params).await?;
    Ok(serde_json::from_value::<UnloadwalletResponse>(raw)?)
}
