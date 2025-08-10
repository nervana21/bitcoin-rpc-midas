//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// has been replaced by the -generate cli option. Refer to -help for more information.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::generate;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.generate().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};

/// Calls the `generate` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn generate(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("generate", &params).await?;
    Ok(raw)
}
