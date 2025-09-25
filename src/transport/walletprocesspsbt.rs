//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.walletprocesspsbt(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::walletprocesspsbt;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = walletprocesspsbt(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletprocesspsbtResponse {
    pub psbt: String,
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

/// Calls the `walletprocesspsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn walletprocesspsbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
    sign: serde_json::Value,
    sighashtype: serde_json::Value,
    bip32derivs: serde_json::Value,
    finalize: serde_json::Value,
) -> Result<WalletprocesspsbtResponse, TransportError> {
    let params =
        vec![json!(psbt), json!(sign), json!(sighashtype), json!(bip32derivs), json!(finalize)];
    let raw = transport.send_request("walletprocesspsbt", &params).await?;
    Ok(serde_json::from_value::<WalletprocesspsbtResponse>(raw)?)
}
