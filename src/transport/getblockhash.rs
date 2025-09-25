//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Returns hash of block in best-block-chain at height provided.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.getblockhash(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::getblockhash;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = getblockhash(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Returns hash of block in best-block-chain at height provided.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetblockhashResponse(pub bitcoin::BlockHash);

/// Calls the `getblockhash` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getblockhash(
    transport: &dyn TransportTrait,
    height: serde_json::Value,
) -> Result<GetblockhashResponse, TransportError> {
    let params = vec![json!(height)];
    let raw = transport.send_request("getblockhash", &params).await?;
    Ok(serde_json::from_value::<GetblockhashResponse>(raw)?)
}
