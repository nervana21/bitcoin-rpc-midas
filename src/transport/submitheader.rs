//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde_json::json;
/// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
/// Throws when the header is invalid.

/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.submitheader(/* params */).await?;
/// # Ok(())
/// # }
/// ```

/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::submitheader;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = submitheader(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```

#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};

/// Calls the `submitheader` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn submitheader(
    transport: &dyn TransportTrait,
    hexdata: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(hexdata)];
    let raw = transport.send_request("submitheader", &params).await?;
    Ok(raw)
}
