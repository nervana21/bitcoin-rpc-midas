//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Verifies blockchain database.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::verifychain;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.verifychain(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Verifies blockchain database.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VerifychainResponse(pub bool);

/// Calls the `verifychain` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn verifychain(
    transport: &dyn TransportTrait,
    checklevel: serde_json::Value,
    nblocks: serde_json::Value,
) -> Result<VerifychainResponse, TransportError> {
    let params = vec![json!(checklevel), json!(nblocks)];
    let raw = transport.send_request("verifychain", &params).await?;
    Ok(serde_json::from_value::<VerifychainResponse>(raw)?)
}
