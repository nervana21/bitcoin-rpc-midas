//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Send a transaction.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::send;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.send(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `send` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
}



/// Calls the `send` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn send(transport: &dyn Transport, outputs: serde_json::Value, conf_target: serde_json::Value, estimate_mode: serde_json::Value, fee_rate: serde_json::Value, options: serde_json::Value) -> Result<SendResponse, TransportError> {
    let params = vec![json!(outputs), json!(conf_target), json!(estimate_mode), json!(fee_rate), json!(options)];
    let raw = transport.send_request("send", &params).await?;
    Ok(serde_json::from_value::<SendResponse>(raw)?)
}
