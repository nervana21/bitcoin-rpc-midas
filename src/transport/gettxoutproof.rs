//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns a hex-encoded proof that "txid" was included in a block.
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::gettxoutproof;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.gettxoutproof(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `gettxoutproof` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettxoutproofResponse {
    /// A string that is a serialized, hex-encoded data for the proof.
    pub data: String,
}



/// Calls the `gettxoutproof` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettxoutproof(transport: &dyn Transport, txids: serde_json::Value, blockhash: serde_json::Value) -> Result<GettxoutproofResponse, TransportError> {
    let params = vec![json!(txids), json!(blockhash)];
    let raw = transport.send_request("gettxoutproof", &params).await?;
    Ok(serde_json::from_value::<GettxoutproofResponse>(raw)?)
}
