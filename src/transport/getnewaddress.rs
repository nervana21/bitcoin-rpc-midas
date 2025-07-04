//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns a new Bitcoin address for receiving payments.
/// If 'label' is specified, it is added to the address book
/// so payments received with the address will be associated with 'label'.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getnewaddress;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getnewaddress(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns a new Bitcoin address for receiving payments.
    /// If 'label' is specified, it is added to the address book
    /// so payments received with the address will be associated with 'label'.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetnewaddressResponse(pub String);



/// Calls the `getnewaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getnewaddress(transport: &dyn TransportTrait, label: serde_json::Value, address_type: serde_json::Value) -> Result<GetnewaddressResponse, TransportError> {
    let params = vec![json!(label), json!(address_type)];
    let raw = transport.send_request("getnewaddress", &params).await?;
    Ok(serde_json::from_value::<GetnewaddressResponse>(raw)?)
}
