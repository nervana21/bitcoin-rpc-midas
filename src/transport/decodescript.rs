//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Decode a hex-encoded script.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::decodescript;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.decodescript(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `decodescript` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecodescriptResponse {
    /// Disassembly of the script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// The output type (e.g. nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    pub r#type: String,
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// address of P2SH script wrapping this redeem script (not returned for types that should not be wrapped)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2sh: Option<String>,
    /// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segwit: Option<serde_json::Value>,
}



/// Calls the `decodescript` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn decodescript(transport: &dyn Transport, hexstring: serde_json::Value) -> Result<DecodescriptResponse, TransportError> {
    let params = vec![json!(hexstring)];
    let raw = transport.send_request("decodescript", &params).await?;
    Ok(serde_json::from_value::<DecodescriptResponse>(raw)?)
}
