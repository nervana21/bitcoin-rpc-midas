//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::verifytxoutproof;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.verifytxoutproof(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `verifytxoutproof` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifytxoutproofResponse {
    pub field_0: Vec<serde_json::Value>,
}



/// Calls the `verifytxoutproof` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn verifytxoutproof(transport: &dyn Transport, proof: serde_json::Value) -> Result<VerifytxoutproofResponse, TransportError> {
    let params = vec![json!(proof)];
    let raw = transport.send_request("verifytxoutproof", &params).await?;
    Ok(serde_json::from_value::<VerifytxoutproofResponse>(raw)?)
}
