//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Submit a raw transaction (serialized, hex-encoded) to local node and network.
/// The transaction will be sent unconditionally to all peers, so using sendrawtransaction
/// for manual rebroadcast may degrade privacy by leaking the transaction's origin, as
/// nodes will normally not rebroadcast non-wallet transactions already in their mempool.
/// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
/// Related RPCs: createrawtransaction, signrawtransactionwithkey

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::sendrawtransaction;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.sendrawtransaction(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `sendrawtransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendrawtransactionResponse {
    /// The transaction hash in hex
    pub field_0: String,
}



/// Calls the `sendrawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn sendrawtransaction(transport: &dyn Transport, hexstring: serde_json::Value, maxfeerate: serde_json::Value, maxburnamount: serde_json::Value) -> Result<SendrawtransactionResponse, TransportError> {
    let params = vec![json!(hexstring), json!(maxfeerate), json!(maxburnamount)];
    let raw = transport.send_request("sendrawtransaction", &params).await?;
    Ok(serde_json::from_value::<SendrawtransactionResponse>(raw)?)
}
