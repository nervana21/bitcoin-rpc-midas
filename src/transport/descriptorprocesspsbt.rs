//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::descriptorprocesspsbt;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.descriptorprocesspsbt(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
#[derive(Debug, Deserialize, Serialize)]
pub struct DescriptorprocesspsbtResponse {
    pub psbt: String,
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

/// Calls the `descriptorprocesspsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn descriptorprocesspsbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
    descriptors: serde_json::Value,
    sighashtype: serde_json::Value,
    bip32derivs: serde_json::Value,
    finalize: serde_json::Value,
) -> Result<DescriptorprocesspsbtResponse, TransportError> {
    let params = vec![
        json!(psbt),
        json!(descriptors),
        json!(sighashtype),
        json!(bip32derivs),
        json!(finalize),
    ];
    let raw = transport
        .send_request("descriptorprocesspsbt", &params)
        .await?;
    Ok(serde_json::from_value::<DescriptorprocesspsbtResponse>(
        raw,
    )?)
}
