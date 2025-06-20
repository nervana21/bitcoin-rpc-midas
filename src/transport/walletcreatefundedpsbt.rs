//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::walletcreatefundedpsbt;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.walletcreatefundedpsbt(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Creates and funds a transaction in the Partially Signed Transaction format.
    /// Implements the Creator and Updater roles.
    /// All existing inputs must either have their previous output transaction be in the wallet
    /// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletcreatefundedpsbtResponse {
    pub psbt: String,
    pub fee: serde_json::Value,
    pub changepos: f64,
}



/// Calls the `walletcreatefundedpsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn walletcreatefundedpsbt(transport: &dyn TransportTrait, inputs: serde_json::Value, outputs: serde_json::Value, locktime: serde_json::Value, options: serde_json::Value, bip32derivs: serde_json::Value) -> Result<WalletcreatefundedpsbtResponse, TransportError> {
    let params = vec![json!(inputs), json!(outputs), json!(locktime), json!(options), json!(bip32derivs)];
    let raw = transport.send_request("walletcreatefundedpsbt", &params).await?;
    Ok(serde_json::from_value::<WalletcreatefundedpsbtResponse>(raw)?)
}
