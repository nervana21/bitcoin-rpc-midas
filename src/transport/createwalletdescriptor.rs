//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::createwalletdescriptor;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.createwalletdescriptor(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatewalletdescriptorResponse {
    pub descs: Vec<serde_json::Value>,
}

/// Calls the `createwalletdescriptor` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn createwalletdescriptor(
    transport: &dyn TransportTrait,
    r#type: serde_json::Value,
    options: serde_json::Value,
) -> Result<CreatewalletdescriptorResponse, TransportError> {
    let params = vec![json!(r#type), json!(options)];
    let raw = transport
        .send_request("createwalletdescriptor", &params)
        .await?;
    Ok(serde_json::from_value::<CreatewalletdescriptorResponse>(
        raw,
    )?)
}
