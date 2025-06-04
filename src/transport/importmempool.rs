//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::importmempool;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.importmempool(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};


/// Calls the `importmempool` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn importmempool(transport: &dyn Transport, filepath: serde_json::Value, options: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(filepath), json!(options)];
    let raw = transport.send_request("importmempool", &params).await?;
    Ok(raw)
}
