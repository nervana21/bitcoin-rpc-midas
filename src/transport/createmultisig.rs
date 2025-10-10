//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Creates a multi-signature address with n signatures of m keys required.
/// It returns a json object with the address and redeemScript.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.createmultisig(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::createmultisig;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = createmultisig(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Creates a multi-signature address with n signatures of m keys required.
/// It returns a json object with the address and redeemScript.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatemultisigResponse {
    pub address: String,
    #[serde(rename = "redeemScript")]
    pub redeem_script: String,
    pub descriptor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}

/// Calls the `createmultisig` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn createmultisig(
    transport: &dyn TransportTrait,
    nrequired: serde_json::Value,
    keys: serde_json::Value,
    address_type: serde_json::Value,
) -> Result<CreatemultisigResponse, TransportError> {
    let params = vec![json!(nrequired), json!(keys), json!(address_type)];
    let raw = transport.send_request("createmultisig", &params).await?;
    Ok(serde_json::from_value::<CreatemultisigResponse>(raw)?)
}
