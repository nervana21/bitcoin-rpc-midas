//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptWitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::finalizepsbt;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.finalizepsbt(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
    /// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
    /// created which has the final_scriptSig and final_scriptWitness fields filled for inputs that are complete.
    /// Implements the Finalizer and Extractor roles.
#[derive(Debug, Deserialize, Serialize)]
pub struct FinalizepsbtResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    pub complete: bool,
}



/// Calls the `finalizepsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn finalizepsbt(transport: &dyn TransportTrait, psbt: serde_json::Value, extract: serde_json::Value) -> Result<FinalizepsbtResponse, TransportError> {
    let params = vec![json!(psbt), json!(extract)];
    let raw = transport.send_request("finalizepsbt", &params).await?;
    Ok(serde_json::from_value::<FinalizepsbtResponse>(raw)?)
}
