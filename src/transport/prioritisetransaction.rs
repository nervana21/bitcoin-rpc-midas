//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Accepts the transaction into mined blocks at a higher (or lower) priority

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.prioritisetransaction(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::prioritisetransaction;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = prioritisetransaction(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Accepts the transaction into mined blocks at a higher (or lower) priority
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PrioritisetransactionResponse(pub bool);

/// Calls the `prioritisetransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn prioritisetransaction(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    dummy: serde_json::Value,
    fee_delta: serde_json::Value,
) -> Result<PrioritisetransactionResponse, TransportError> {
    let params = vec![json!(txid), json!(dummy), json!(fee_delta)];
    let raw = transport.send_request("prioritisetransaction", &params).await?;
    Ok(serde_json::from_value::<PrioritisetransactionResponse>(raw)?)
}
