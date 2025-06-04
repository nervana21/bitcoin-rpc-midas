//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::walletprocesspsbt;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.walletprocesspsbt(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `walletprocesspsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletprocesspsbtResponse {
    /// The base64-encoded partially signed transaction
    pub psbt: String,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}



/// Calls the `walletprocesspsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn walletprocesspsbt(transport: &dyn Transport, psbt: serde_json::Value, sign: serde_json::Value, sighashtype: serde_json::Value, bip32derivs: serde_json::Value, finalize: serde_json::Value) -> Result<WalletprocesspsbtResponse, TransportError> {
    let params = vec![json!(psbt), json!(sign), json!(sighashtype), json!(bip32derivs), json!(finalize)];
    let raw = transport.send_request("walletprocesspsbt", &params).await?;
    Ok(serde_json::from_value::<WalletprocesspsbtResponse>(raw)?)
}
