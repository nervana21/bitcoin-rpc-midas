//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptWitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.finalizepsbt(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::finalizepsbt;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = finalizepsbt(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptWitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
#[derive(Debug, Deserialize, Serialize)]
pub struct FinalizepsbtResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    pub complete: bool,
}

/// Calls the `finalizepsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn finalizepsbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
    extract: serde_json::Value,
) -> Result<FinalizepsbtResponse, TransportError> {
    let params = vec![json!(psbt), json!(extract)];
    let raw = transport.send_request("finalizepsbt", &params).await?;
    Ok(serde_json::from_value::<FinalizepsbtResponse>(raw)?)
}
