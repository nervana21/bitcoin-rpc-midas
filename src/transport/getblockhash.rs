//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Returns hash of block in best-block-chain at height provided.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getblockhash;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getblockhash(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns hash of block in best-block-chain at height provided.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetblockhashResponse(pub bitcoin::BlockHash);

/// Calls the `getblockhash` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockhash(
    transport: &dyn TransportTrait,
    height: serde_json::Value,
) -> Result<GetblockhashResponse, TransportError> {
    let params = vec![json!(height)];
    let raw = transport.send_request("getblockhash", &params).await?;
    Ok(serde_json::from_value::<GetblockhashResponse>(raw)?)
}
