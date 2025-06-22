//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::walletprocesspsbt;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.walletprocesspsbt(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
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
pub async fn walletprocesspsbt(transport: &dyn TransportTrait, psbt: serde_json::Value, sign: serde_json::Value, sighashtype: serde_json::Value, bip32derivs: serde_json::Value, finalize: serde_json::Value) -> Result<WalletprocesspsbtResponse, TransportError> {
    let params = vec![json!(psbt), json!(sign), json!(sighashtype), json!(bip32derivs), json!(finalize)];
    let raw = transport.send_request("walletprocesspsbt", &params).await?;
    Ok(serde_json::from_value::<WalletprocesspsbtResponse>(raw)?)
}
