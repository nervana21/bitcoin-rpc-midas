//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.signrawtransactionwithkey(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::signrawtransactionwithkey;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = signrawtransactionwithkey(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
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
    let params = vec![json!(hexstring), json!(privkeys), json!(prevtxs), json!(sighashtype)];
    let raw = transport.send_request("signrawtransactionwithkey", &params).await?;
    Ok(serde_json::from_value::<SignrawtransactionwithkeyResponse>(raw)?)
}
