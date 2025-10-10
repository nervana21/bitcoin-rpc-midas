//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.sendtoaddress(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::sendtoaddress;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = sendtoaddress(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SendtoaddressResponse {
    Variant1(bitcoin::Txid),
    Variant2 { txid: bitcoin::Txid, fee_reason: String },
}

/// Calls the `sendtoaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
#[allow(clippy::too_many_arguments)]
pub async fn sendtoaddress(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    amount: serde_json::Value,
    comment: serde_json::Value,
    comment_to: serde_json::Value,
    subtractfeefromamount: serde_json::Value,
    replaceable: serde_json::Value,
    conf_target: serde_json::Value,
    estimate_mode: serde_json::Value,
    avoid_reuse: serde_json::Value,
    fee_rate: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<SendtoaddressResponse, TransportError> {
    let params = vec![
        json!(address),
        json!(amount),
        json!(comment),
        json!(comment_to),
        json!(subtractfeefromamount),
        json!(replaceable),
        json!(conf_target),
        json!(estimate_mode),
        json!(avoid_reuse),
        json!(fee_rate),
        json!(verbose),
    ];
    let raw = transport.send_request("sendtoaddress", &params).await?;
    Ok(serde_json::from_value::<SendtoaddressResponse>(raw)?)
}
