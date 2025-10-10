//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.sendmany(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::sendmany;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = sendmany(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SendmanyResponse {
    Variant1(bitcoin::Txid),
    Variant2 { txid: bitcoin::Txid, fee_reason: String },
}

/// Calls the `sendmany` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
#[allow(clippy::too_many_arguments)]
pub async fn sendmany(
    transport: &dyn TransportTrait,
    dummy: serde_json::Value,
    amounts: serde_json::Value,
    minconf: serde_json::Value,
    comment: serde_json::Value,
    subtractfeefrom: serde_json::Value,
    replaceable: serde_json::Value,
    conf_target: serde_json::Value,
    estimate_mode: serde_json::Value,
    fee_rate: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<SendmanyResponse, TransportError> {
    let params = vec![
        json!(dummy),
        json!(amounts),
        json!(minconf),
        json!(comment),
        json!(subtractfeefrom),
        json!(replaceable),
        json!(conf_target),
        json!(estimate_mode),
        json!(fee_rate),
        json!(verbose),
    ];
    let raw = transport.send_request("sendmany", &params).await?;
    Ok(serde_json::from_value::<SendmanyResponse>(raw)?)
}
