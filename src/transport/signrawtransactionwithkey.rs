//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::signrawtransactionwithkey;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.signrawtransactionwithkey(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignrawtransactionwithkeyResponse {
    pub hex: String,
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<serde_json::Value>>,
}

/// Calls the `signrawtransactionwithkey` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn signrawtransactionwithkey(
    transport: &dyn TransportTrait,
    hexstring: serde_json::Value,
    privkeys: serde_json::Value,
    prevtxs: serde_json::Value,
    sighashtype: serde_json::Value,
) -> Result<SignrawtransactionwithkeyResponse, TransportError> {
    let params = vec![
        json!(hexstring),
        json!(privkeys),
        json!(prevtxs),
        json!(sighashtype),
    ];
    let raw = transport
        .send_request("signrawtransactionwithkey", &params)
        .await?;
    Ok(serde_json::from_value::<SignrawtransactionwithkeyResponse>(
        raw,
    )?)
}
