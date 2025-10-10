//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns details about an unspent transaction output.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.gettxout(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::gettxout;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = gettxout(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns details about an unspent transaction output.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GettxoutResponse {
    Variant2 {
        bestblock: String,
        confirmations: u64,
        value: f64,
        #[serde(rename = "scriptPubKey")]
        script_pubkey: serde_json::Value,
        coinbase: bool,
    },
}

/// Calls the `gettxout` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettxout(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    n: serde_json::Value,
    include_mempool: serde_json::Value,
) -> Result<GettxoutResponse, TransportError> {
    let params = vec![json!(txid), json!(n), json!(include_mempool)];
    let raw = transport.send_request("gettxout", &params).await?;
    Ok(serde_json::from_value::<GettxoutResponse>(raw)?)
}
