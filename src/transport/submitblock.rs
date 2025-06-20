//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Attempts to submit new block to network.
/// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::submitblock;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.submitblock(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Attempts to submit new block to network.
    /// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubmitblockResponse(pub serde_json::Value);



/// Calls the `submitblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn submitblock(transport: &dyn TransportTrait, hexdata: serde_json::Value, dummy: serde_json::Value) -> Result<SubmitblockResponse, TransportError> {
    let params = vec![json!(hexdata), json!(dummy)];
    let raw = transport.send_request("submitblock", &params).await?;
    Ok(serde_json::from_value::<SubmitblockResponse>(raw)?)
}
