//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Returns a hex-encoded proof that "txid" was included in a block.
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::gettxoutproof;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.gettxoutproof(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns a hex-encoded proof that \"txid\" was included in a block.
///
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GettxoutproofResponse(pub String);

/// Calls the `gettxoutproof` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettxoutproof(
    transport: &dyn TransportTrait,
    txids: serde_json::Value,
    blockhash: serde_json::Value,
) -> Result<GettxoutproofResponse, TransportError> {
    let params = vec![json!(txids), json!(blockhash)];
    let raw = transport.send_request("gettxoutproof", &params).await?;
    Ok(serde_json::from_value::<GettxoutproofResponse>(raw)?)
}
