//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// List all descriptors present in a wallet.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::listdescriptors;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.listdescriptors(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// List all descriptors present in a wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListdescriptorsResponse {
    pub wallet_name: String,
    pub descriptors: Vec<serde_json::Value>,
}

/// Calls the `listdescriptors` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listdescriptors(
    transport: &dyn TransportTrait,
    private: serde_json::Value,
) -> Result<ListdescriptorsResponse, TransportError> {
    let params = vec![json!(private)];
    let raw = transport.send_request("listdescriptors", &params).await?;
    Ok(serde_json::from_value::<ListdescriptorsResponse>(raw)?)
}
