//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.walletcreatefundedpsbt(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::walletcreatefundedpsbt;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = walletcreatefundedpsbt(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletcreatefundedpsbtResponse {
    pub psbt: String,
    pub fee: f64,
    pub changepos: u64,
}

/// Calls the `walletcreatefundedpsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn walletcreatefundedpsbt(
    transport: &dyn TransportTrait,
    inputs: serde_json::Value,
    outputs: serde_json::Value,
    locktime: serde_json::Value,
    options: serde_json::Value,
    bip32derivs: serde_json::Value,
    version: serde_json::Value,
) -> Result<WalletcreatefundedpsbtResponse, TransportError> {
    let params = vec![
        json!(inputs),
        json!(outputs),
        json!(locktime),
        json!(options),
        json!(bip32derivs),
        json!(version),
    ];
    let raw = transport.send_request("walletcreatefundedpsbt", &params).await?;
    Ok(serde_json::from_value::<WalletcreatefundedpsbtResponse>(raw)?)
}
