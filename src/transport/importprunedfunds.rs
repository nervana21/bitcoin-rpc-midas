//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::importprunedfunds;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.importprunedfunds(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `importprunedfunds` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn importprunedfunds(transport: &dyn TransportTrait, rawtransaction: serde_json::Value, txoutproof: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(rawtransaction), json!(txoutproof)];
    let raw = transport.send_request("importprunedfunds", &params).await?;
    Ok(raw)
}
