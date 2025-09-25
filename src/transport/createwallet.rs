//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Creates and loads a new wallet.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.createwallet(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::createwallet;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = createwallet(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Creates and loads a new wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatewalletResponse {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}

/// Calls the `createwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn createwallet(
    transport: &dyn TransportTrait,
    wallet_name: serde_json::Value,
    disable_private_keys: serde_json::Value,
    blank: serde_json::Value,
    passphrase: serde_json::Value,
    avoid_reuse: serde_json::Value,
    descriptors: serde_json::Value,
    load_on_startup: serde_json::Value,
    external_signer: serde_json::Value,
) -> Result<CreatewalletResponse, TransportError> {
    let params = vec![
        json!(wallet_name),
        json!(disable_private_keys),
        json!(blank),
        json!(passphrase),
        json!(avoid_reuse),
        json!(descriptors),
        json!(load_on_startup),
        json!(external_signer),
    ];
    let raw = transport.send_request("createwallet", &params).await?;
    Ok(serde_json::from_value::<CreatewalletResponse>(raw)?)
}
