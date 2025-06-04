//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::descriptorprocesspsbt;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.descriptorprocesspsbt(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `descriptorprocesspsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DescriptorprocesspsbtResponse {
    /// The base64-encoded partially signed transaction
    pub psbt: String,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}



/// Calls the `descriptorprocesspsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn descriptorprocesspsbt(transport: &dyn Transport, psbt: serde_json::Value, descriptors: serde_json::Value, sighashtype: serde_json::Value, bip32derivs: serde_json::Value, finalize: serde_json::Value) -> Result<DescriptorprocesspsbtResponse, TransportError> {
    let params = vec![json!(psbt), json!(descriptors), json!(sighashtype), json!(bip32derivs), json!(finalize)];
    let raw = transport.send_request("descriptorprocesspsbt", &params).await?;
    Ok(serde_json::from_value::<DescriptorprocesspsbtResponse>(raw)?)
}
