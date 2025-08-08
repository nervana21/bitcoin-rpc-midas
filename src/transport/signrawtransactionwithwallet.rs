//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::signrawtransactionwithwallet;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.signrawtransactionwithwallet(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignrawtransactionwithwalletResponse {
    pub hex: String,
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<serde_json::Value>>,
}

/// Calls the `signrawtransactionwithwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn signrawtransactionwithwallet(
    transport: &dyn TransportTrait,
    hexstring: serde_json::Value,
    prevtxs: serde_json::Value,
    sighashtype: serde_json::Value,
) -> Result<SignrawtransactionwithwalletResponse, TransportError> {
    let params = vec![json!(hexstring), json!(prevtxs), json!(sighashtype)];
    let raw = transport
        .send_request("signrawtransactionwithwallet", &params)
        .await?;
    Ok(serde_json::from_value::<
        SignrawtransactionwithwalletResponse,
    >(raw)?)
}
