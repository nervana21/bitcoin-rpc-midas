//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Mine to a specified address and return the block hashes.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::generatetoaddress;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.generatetoaddress(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Mine to a specified address and return the block hashes.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GeneratetoaddressResponse(pub Vec<serde_json::Value>);



/// Calls the `generatetoaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn generatetoaddress(transport: &dyn TransportTrait, nblocks: serde_json::Value, address: serde_json::Value, maxtries: serde_json::Value) -> Result<GeneratetoaddressResponse, TransportError> {
    let params = vec![json!(nblocks), json!(address), json!(maxtries)];
    let raw = transport.send_request("generatetoaddress", &params).await?;
    Ok(serde_json::from_value::<GeneratetoaddressResponse>(raw)?)
}
