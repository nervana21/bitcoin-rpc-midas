//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Submit a raw transaction (serialized, hex-encoded) to local node and network.
/// The transaction will be sent unconditionally to all peers, so using sendrawtransaction
/// for manual rebroadcast may degrade privacy by leaking the transaction's origin, as
/// nodes will normally not rebroadcast non-wallet transactions already in their mempool.
/// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
/// Related RPCs: createrawtransaction, signrawtransactionwithkey
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.sendrawtransaction(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::sendrawtransaction;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = sendrawtransaction(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Submit a raw transaction (serialized, hex-encoded) to local node and network.
///
/// The transaction will be sent unconditionally to all peers, so using sendrawtransaction
/// for manual rebroadcast may degrade privacy by leaking the transaction's origin, as
/// nodes will normally not rebroadcast non-wallet transactions already in their mempool.
///
/// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
///
/// Related RPCs: createrawtransaction, signrawtransactionwithkey
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SendrawtransactionResponse(pub String);

/// Calls the `sendrawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn sendrawtransaction(
    transport: &dyn TransportTrait,
    hexstring: serde_json::Value,
    maxfeerate: serde_json::Value,
    maxburnamount: serde_json::Value,
) -> Result<SendrawtransactionResponse, TransportError> {
    let params = vec![json!(hexstring), json!(maxfeerate), json!(maxburnamount)];
    let raw = transport.send_request("sendrawtransaction", &params).await?;
    Ok(serde_json::from_value::<SendrawtransactionResponse>(raw)?)
}
