//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns hash of block in best-block-chain at height provided.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getblockhash;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getblockhash(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getblockhash` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockhashResponse {
    /// The block hash
    pub field_0: String,
}



/// Calls the `getblockhash` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockhash(transport: &dyn Transport, height: serde_json::Value) -> Result<GetblockhashResponse, TransportError> {
    let params = vec![json!(height)];
    let raw = transport.send_request("getblockhash", &params).await?;
    Ok(serde_json::from_value::<GetblockhashResponse>(raw)?)
}
