//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Accepts the transaction into mined blocks at a higher (or lower) priority

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::prioritisetransaction;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.prioritisetransaction(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `prioritisetransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PrioritisetransactionResponse {
    /// Returns true
    pub field_0: bool,
}



/// Calls the `prioritisetransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn prioritisetransaction(transport: &dyn Transport, txid: serde_json::Value, dummy: serde_json::Value, fee_delta: serde_json::Value) -> Result<PrioritisetransactionResponse, TransportError> {
    let params = vec![json!(txid), json!(dummy), json!(fee_delta)];
    let raw = transport.send_request("prioritisetransaction", &params).await?;
    Ok(serde_json::from_value::<PrioritisetransactionResponse>(raw)?)
}
