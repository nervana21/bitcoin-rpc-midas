//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// has been replaced by the -generate cli option. Refer to -help for more information.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::generate;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.generate().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `generate` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn generate(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("generate", &params).await?;
    Ok(raw)
}
