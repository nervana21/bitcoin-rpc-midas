//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::utxoupdatepsbt;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.utxoupdatepsbt(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `utxoupdatepsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UtxoupdatepsbtResponse {
    /// The base64-encoded partially signed transaction with inputs updated
    pub field_0: String,
}



/// Calls the `utxoupdatepsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn utxoupdatepsbt(transport: &dyn Transport, psbt: serde_json::Value, descriptors: serde_json::Value) -> Result<UtxoupdatepsbtResponse, TransportError> {
    let params = vec![json!(psbt), json!(descriptors)];
    let raw = transport.send_request("utxoupdatepsbt", &params).await?;
    Ok(serde_json::from_value::<UtxoupdatepsbtResponse>(raw)?)
}
