//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// List all descriptors present in a wallet.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.listdescriptors(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::listdescriptors;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = listdescriptors(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// List all descriptors present in a wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListdescriptorsResponse {
    pub wallet_name: String,
    pub descriptors: Vec<serde_json::Value>,
}

/// Calls the `listdescriptors` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listdescriptors(
    transport: &dyn TransportTrait,
    private: serde_json::Value,
) -> Result<ListdescriptorsResponse, TransportError> {
    let params = vec![json!(private)];
    let raw = transport.send_request("listdescriptors", &params).await?;
    Ok(serde_json::from_value::<ListdescriptorsResponse>(raw)?)
}
