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
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.sendall(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// EXPERIMENTAL warning: this call may be changed in future releases.
    /// 
    /// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
    /// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
    /// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
#[derive(Debug, Deserialize, Serialize)]
pub struct SendallResponse {
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
}



/// Calls the `sendall` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn sendall(transport: &dyn TransportTrait, recipients: serde_json::Value, conf_target: serde_json::Value, estimate_mode: serde_json::Value, fee_rate: serde_json::Value, options: serde_json::Value) -> Result<SendallResponse, TransportError> {
    let params = vec![json!(recipients), json!(conf_target), json!(estimate_mode), json!(fee_rate), json!(options)];
    let raw = transport.send_request("sendall", &params).await?;
    Ok(serde_json::from_value::<SendallResponse>(raw)?)
}
