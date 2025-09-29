//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns a hex-encoded proof that "txid" was included in a block.
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.gettxoutproof(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::gettxoutproof;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = gettxoutproof(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns a hex-encoded proof that \"txid\" was included in a block.
///
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GettxoutproofResponse(pub String);

/// Calls the `gettxoutproof` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn gettxoutproof(
    transport: &dyn TransportTrait,
    txids: serde_json::Value,
    blockhash: serde_json::Value,
) -> Result<GettxoutproofResponse, TransportError> {
    let params = vec![json!(txids), json!(blockhash)];
    let raw = transport.send_request("gettxoutproof", &params).await?;
    Ok(serde_json::from_value::<GettxoutproofResponse>(raw)?)
}
