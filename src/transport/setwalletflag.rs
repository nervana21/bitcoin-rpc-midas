//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Change the state of the given wallet flag for a wallet.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.setwalletflag(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::setwalletflag;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = setwalletflag(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Change the state of the given wallet flag for a wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct SetwalletflagResponse {
    pub flag_name: String,
    pub flag_state: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<String>,
}

/// Calls the `setwalletflag` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn setwalletflag(
    transport: &dyn TransportTrait,
    flag: serde_json::Value,
    value: serde_json::Value,
) -> Result<SetwalletflagResponse, TransportError> {
    let params = vec![json!(flag), json!(value)];
    let raw = transport.send_request("setwalletflag", &params).await?;
    Ok(serde_json::from_value::<SetwalletflagResponse>(raw)?)
}
