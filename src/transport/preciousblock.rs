//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde_json::json;
/// Treats a block as if it were received before others with the same work.
/// A later preciousblock call can override the effect of an earlier one.
/// The effects of preciousblock are not retained across restarts.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.preciousblock(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::preciousblock;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = preciousblock(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};

/// Calls the `preciousblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn preciousblock(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(blockhash)];
    let raw = transport.send_request("preciousblock", &params).await?;
    Ok(raw)
}
