//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getrawchangeaddress;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getrawchangeaddress(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getrawchangeaddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrawchangeaddressResponse {
    /// The address
    pub address: String,
}



/// Calls the `getrawchangeaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getrawchangeaddress(transport: &dyn Transport, address_type: serde_json::Value) -> Result<GetrawchangeaddressResponse, TransportError> {
    let params = vec![json!(address_type)];
    let raw = transport.send_request("getrawchangeaddress", &params).await?;
    Ok(serde_json::from_value::<GetrawchangeaddressResponse>(raw)?)
}
