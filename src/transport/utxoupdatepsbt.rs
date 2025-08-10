//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::utxoupdatepsbt;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.utxoupdatepsbt(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtxoupdatepsbtResponse(pub String);

/// Calls the `utxoupdatepsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn utxoupdatepsbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
    descriptors: serde_json::Value,
) -> Result<UtxoupdatepsbtResponse, TransportError> {
    let params = vec![json!(psbt), json!(descriptors)];
    let raw = transport.send_request("utxoupdatepsbt", &params).await?;
    Ok(serde_json::from_value::<UtxoupdatepsbtResponse>(raw)?)
}
