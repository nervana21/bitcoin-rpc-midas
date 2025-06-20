//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// List descriptors imported into a descriptor-enabled wallet.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::listdescriptors;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.listdescriptors(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// List descriptors imported into a descriptor-enabled wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListdescriptorsResponse {
    pub wallet_name: String,
    pub descriptors: Vec<serde_json::Value>,
}



/// Calls the `listdescriptors` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listdescriptors(transport: &dyn TransportTrait, private: serde_json::Value) -> Result<ListdescriptorsResponse, TransportError> {
    let params = vec![json!(private)];
    let raw = transport.send_request("listdescriptors", &params).await?;
    Ok(serde_json::from_value::<ListdescriptorsResponse>(raw)?)
}
