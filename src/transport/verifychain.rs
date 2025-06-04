//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Verifies blockchain database.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::verifychain;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.verifychain(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `verifychain` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifychainResponse {
    /// Verification finished successfully. If false, check debug.log for reason.
    pub field_0: bool,
}



/// Calls the `verifychain` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn verifychain(transport: &dyn Transport, checklevel: serde_json::Value, nblocks: serde_json::Value) -> Result<VerifychainResponse, TransportError> {
    let params = vec![json!(checklevel), json!(nblocks)];
    let raw = transport.send_request("verifychain", &params).await?;
    Ok(serde_json::from_value::<VerifychainResponse>(raw)?)
}
