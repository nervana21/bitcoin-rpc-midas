//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Reveals the private key corresponding to 'address'.
/// Then the importprivkey can be used with this output
/// Note: This command is only compatible with legacy wallets.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::dumpprivkey;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.dumpprivkey(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Reveals the private key corresponding to 'address'.
    /// Then the importprivkey can be used with this output
    /// Note: This command is only compatible with legacy wallets.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DumpprivkeyResponse(pub String);



/// Calls the `dumpprivkey` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn dumpprivkey(transport: &dyn TransportTrait, address: serde_json::Value) -> Result<DumpprivkeyResponse, TransportError> {
    let params = vec![json!(address)];
    let raw = transport.send_request("dumpprivkey", &params).await?;
    Ok(serde_json::from_value::<DumpprivkeyResponse>(raw)?)
}
