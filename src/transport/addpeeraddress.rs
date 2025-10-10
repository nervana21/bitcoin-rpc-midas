//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Add the address of a potential peer to an address manager table. This RPC is for testing only.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.addpeeraddress(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::addpeeraddress;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = addpeeraddress(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Add the address of a potential peer to an address manager table. This RPC is for testing only.
#[derive(Debug, Deserialize, Serialize)]
pub struct AddpeeraddressResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Calls the `addpeeraddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn addpeeraddress(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    port: serde_json::Value,
    tried: serde_json::Value,
) -> Result<AddpeeraddressResponse, TransportError> {
    let params = vec![json!(address), json!(port), json!(tried)];
    let raw = transport.send_request("addpeeraddress", &params).await?;
    Ok(serde_json::from_value::<AddpeeraddressResponse>(raw)?)
}
