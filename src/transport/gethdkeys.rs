//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// List all BIP 32 HD keys in the wallet and which descriptors use them.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::gethdkeys;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.gethdkeys(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// List all BIP 32 HD keys in the wallet and which descriptors use them.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GethdkeysResponse(pub Vec<serde_json::Value>);



/// Calls the `gethdkeys` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gethdkeys(transport: &dyn TransportTrait, options: serde_json::Value) -> Result<GethdkeysResponse, TransportError> {
    let params = vec![json!(options)];
    let raw = transport.send_request("gethdkeys", &params).await?;
    Ok(serde_json::from_value::<GethdkeysResponse>(raw)?)
}
