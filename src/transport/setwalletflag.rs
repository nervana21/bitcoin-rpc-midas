//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Change the state of the given wallet flag for a wallet.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::setwalletflag;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.setwalletflag(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Change the state of the given wallet flag for a wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct SetwalletflagResponse {
    pub flag_name: String,
    pub flag_state: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<String>,
}



/// Calls the `setwalletflag` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn setwalletflag(transport: &dyn TransportTrait, flag: serde_json::Value, value: serde_json::Value) -> Result<SetwalletflagResponse, TransportError> {
    let params = vec![json!(flag), json!(value)];
    let raw = transport.send_request("setwalletflag", &params).await?;
    Ok(serde_json::from_value::<SetwalletflagResponse>(raw)?)
}
