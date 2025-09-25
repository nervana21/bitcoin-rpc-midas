//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.createwalletdescriptor(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::createwalletdescriptor;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = createwalletdescriptor(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatewalletdescriptorResponse {
    pub descs: Vec<serde_json::Value>,
}

/// Calls the `createwalletdescriptor` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn createwalletdescriptor(
    transport: &dyn TransportTrait,
    r#type: serde_json::Value,
    options: serde_json::Value,
) -> Result<CreatewalletdescriptorResponse, TransportError> {
    let params = vec![json!(r#type), json!(options)];
    let raw = transport.send_request("createwalletdescriptor", &params).await?;
    Ok(serde_json::from_value::<CreatewalletdescriptorResponse>(raw)?)
}
