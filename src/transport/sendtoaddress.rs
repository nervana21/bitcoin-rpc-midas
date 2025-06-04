//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::sendtoaddress;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.sendtoaddress(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `sendtoaddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendtoaddressResponse {
    /// The transaction id.
    pub txid: bitcoin::Txid,
    pub field_1: serde_json::Value,
}



/// Calls the `sendtoaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn sendtoaddress(transport: &dyn Transport, address: serde_json::Value, amount: serde_json::Value, comment: serde_json::Value, comment_to: serde_json::Value, subtractfeefromamount: serde_json::Value, replaceable: serde_json::Value, conf_target: serde_json::Value, estimate_mode: serde_json::Value, avoid_reuse: serde_json::Value, fee_rate: serde_json::Value, verbose: serde_json::Value) -> Result<SendtoaddressResponse, TransportError> {
    let params = vec![json!(address), json!(amount), json!(comment), json!(comment_to), json!(subtractfeefromamount), json!(replaceable), json!(conf_target), json!(estimate_mode), json!(avoid_reuse), json!(fee_rate), json!(verbose)];
    let raw = transport.send_request("sendtoaddress", &params).await?;
    Ok(serde_json::from_value::<SendtoaddressResponse>(raw)?)
}
