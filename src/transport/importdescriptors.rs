//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
/// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second elements will be imported as an internal descriptor.
/// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::importdescriptors;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.importdescriptors(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `importdescriptors` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ImportdescriptorsResponse {
    /// Response is an array with the same size as the input that has the execution result
    pub field_0: Vec<serde_json::Value>,
}



/// Calls the `importdescriptors` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn importdescriptors(transport: &dyn Transport, requests: serde_json::Value) -> Result<ImportdescriptorsResponse, TransportError> {
    let params = vec![json!(requests)];
    let raw = transport.send_request("importdescriptors", &params).await?;
    Ok(serde_json::from_value::<ImportdescriptorsResponse>(raw)?)
}
