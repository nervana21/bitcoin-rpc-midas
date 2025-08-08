//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Return RPC command JSON Schema descriptions.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::schema;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.schema().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Return RPC command JSON Schema descriptions.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SchemaResponse(pub serde_json::Value);

/// Calls the `schema` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn schema(transport: &dyn TransportTrait) -> Result<SchemaResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("schema", &params).await?;
    Ok(serde_json::from_value::<SchemaResponse>(raw)?)
}
