//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Display address on an external signer for verification.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.walletdisplayaddress(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::walletdisplayaddress;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = walletdisplayaddress(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Display address on an external signer for verification.
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletdisplayaddressResponse {
    pub address: String,
}

/// Calls the `walletdisplayaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn walletdisplayaddress(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
) -> Result<WalletdisplayaddressResponse, TransportError> {
    let params = vec![json!(address)];
    let raw = transport.send_request("walletdisplayaddress", &params).await?;
    Ok(serde_json::from_value::<WalletdisplayaddressResponse>(raw)?)
}
