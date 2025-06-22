//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Display address on an external signer for verification.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::walletdisplayaddress;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.walletdisplayaddress(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Display address on an external signer for verification.
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletdisplayaddressResponse {
    pub address: String,
}



/// Calls the `walletdisplayaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn walletdisplayaddress(transport: &dyn TransportTrait, address: serde_json::Value) -> Result<WalletdisplayaddressResponse, TransportError> {
    let params = vec![json!(address)];
    let raw = transport.send_request("walletdisplayaddress", &params).await?;
    Ok(serde_json::from_value::<WalletdisplayaddressResponse>(raw)?)
}
