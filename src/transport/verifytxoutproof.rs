//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::verifytxoutproof;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.verifytxoutproof(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VerifytxoutproofResponse(pub Vec<serde_json::Value>);

/// Calls the `verifytxoutproof` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn verifytxoutproof(
    transport: &dyn TransportTrait,
    proof: serde_json::Value,
) -> Result<VerifytxoutproofResponse, TransportError> {
    let params = vec![json!(proof)];
    let raw = transport.send_request("verifytxoutproof", &params).await?;
    Ok(serde_json::from_value::<VerifytxoutproofResponse>(raw)?)
}
