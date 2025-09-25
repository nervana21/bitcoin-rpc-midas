//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Attempts to submit new block to network.
/// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.submitblock(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::submitblock;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = submitblock(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Attempts to submit new block to network.
/// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SubmitblockResponse {
    Variant2(String),
}

/// Calls the `submitblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn submitblock(
    transport: &dyn TransportTrait,
    hexdata: serde_json::Value,
    dummy: serde_json::Value,
) -> Result<SubmitblockResponse, TransportError> {
    let params = vec![json!(hexdata), json!(dummy)];
    let raw = transport.send_request("submitblock", &params).await?;
    Ok(serde_json::from_value::<SubmitblockResponse>(raw)?)
}
