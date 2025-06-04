//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::removeprunedfunds;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.removeprunedfunds(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};


/// Calls the `removeprunedfunds` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn removeprunedfunds(transport: &dyn Transport, txid: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(txid)];
    let raw = transport.send_request("removeprunedfunds", &params).await?;
    Ok(raw)
}
