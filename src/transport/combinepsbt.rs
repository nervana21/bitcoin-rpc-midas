//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Combine multiple partially signed Bitcoin transactions into one transaction.
/// Implements the Combiner role.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::combinepsbt;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.combinepsbt(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Combine multiple partially signed Bitcoin transactions into one transaction.
    /// Implements the Combiner role.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CombinepsbtResponse(pub String);



/// Calls the `combinepsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn combinepsbt(transport: &dyn TransportTrait, txs: serde_json::Value) -> Result<CombinepsbtResponse, TransportError> {
    let params = vec![json!(txs)];
    let raw = transport.send_request("combinepsbt", &params).await?;
    Ok(serde_json::from_value::<CombinepsbtResponse>(raw)?)
}
