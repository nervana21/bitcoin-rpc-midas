//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.descriptorprocesspsbt(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::descriptorprocesspsbt;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = descriptorprocesspsbt(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
#[derive(Debug, Deserialize, Serialize)]
pub struct DescriptorprocesspsbtResponse {
    pub psbt: String,
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

/// Calls the `descriptorprocesspsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn descriptorprocesspsbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
    descriptors: serde_json::Value,
    sighashtype: serde_json::Value,
    bip32derivs: serde_json::Value,
    finalize: serde_json::Value,
) -> Result<DescriptorprocesspsbtResponse, TransportError> {
    let params = vec![
        json!(psbt),
        json!(descriptors),
        json!(sighashtype),
        json!(bip32derivs),
        json!(finalize),
    ];
    let raw = transport.send_request("descriptorprocesspsbt", &params).await?;
    Ok(serde_json::from_value::<DescriptorprocesspsbtResponse>(raw)?)
}
