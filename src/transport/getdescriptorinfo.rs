//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Analyses a descriptor.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getdescriptorinfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getdescriptorinfo(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getdescriptorinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetdescriptorinfoResponse {
    /// The descriptor in canonical form, without private keys. For a multipath descriptor, only the first will be returned.
    pub descriptor: String,
    /// All descriptors produced by expanding multipath derivation elements. Only if the provided descriptor specifies multipath derivation elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipath_expansion: Option<Vec<serde_json::Value>>,
    /// The checksum for the input descriptor
    pub checksum: String,
    /// Whether the descriptor is ranged
    pub isrange: bool,
    /// Whether the descriptor is solvable
    pub issolvable: bool,
    /// Whether the input descriptor contained at least one private key
    pub hasprivatekeys: bool,
}



/// Calls the `getdescriptorinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getdescriptorinfo(transport: &dyn Transport, descriptor: serde_json::Value) -> Result<GetdescriptorinfoResponse, TransportError> {
    let params = vec![json!(descriptor)];
    let raw = transport.send_request("getdescriptorinfo", &params).await?;
    Ok(serde_json::from_value::<GetdescriptorinfoResponse>(raw)?)
}
