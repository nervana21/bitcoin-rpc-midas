//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.utxoupdatepsbt(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::utxoupdatepsbt;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = utxoupdatepsbt(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtxoupdatepsbtResponse(pub String);

/// Calls the `utxoupdatepsbt` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn utxoupdatepsbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
    descriptors: serde_json::Value,
) -> Result<UtxoupdatepsbtResponse, TransportError> {
    let params = vec![json!(psbt), json!(descriptors)];
    let raw = transport.send_request("utxoupdatepsbt", &params).await?;
    Ok(serde_json::from_value::<UtxoupdatepsbtResponse>(raw)?)
}
