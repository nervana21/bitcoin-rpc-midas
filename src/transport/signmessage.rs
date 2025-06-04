//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Sign a message with the private key of an address
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::signmessage;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.signmessage(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `signmessage` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SignmessageResponse {
    /// The signature of the message encoded in base 64
    pub signature: String,
}



/// Calls the `signmessage` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn signmessage(transport: &dyn Transport, address: serde_json::Value, message: serde_json::Value) -> Result<SignmessageResponse, TransportError> {
    let params = vec![json!(address), json!(message)];
    let raw = transport.send_request("signmessage", &params).await?;
    Ok(serde_json::from_value::<SignmessageResponse>(raw)?)
}
