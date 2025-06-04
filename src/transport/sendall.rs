//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
/// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::sendall;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.sendall(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `sendall` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendallResponse {
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



/// Calls the `sendall` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn sendall(transport: &dyn Transport, recipients: serde_json::Value, conf_target: serde_json::Value, estimate_mode: serde_json::Value, fee_rate: serde_json::Value, options: serde_json::Value) -> Result<SendallResponse, TransportError> {
    let params = vec![json!(recipients), json!(conf_target), json!(estimate_mode), json!(fee_rate), json!(options)];
    let raw = transport.send_request("sendall", &params).await?;
    Ok(serde_json::from_value::<SendallResponse>(raw)?)
}
