//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Rescan the local blockchain for wallet related transactions.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// The rescan is significantly faster if block filters are available
/// (using startup option "-blockfilterindex=1").
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.rescanblockchain(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::rescanblockchain;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = rescanblockchain(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// Rescan the local blockchain for wallet related transactions.
/// Note: Use \"getwalletinfo\" to query the scanning progress.
/// The rescan is significantly faster if block filters are available
/// (using startup option \"-blockfilterindex=1\").
#[derive(Debug, Deserialize, Serialize)]
pub struct RescanblockchainResponse {
    pub start_height: u64,
    pub stop_height: u64,
}

/// Calls the `rescanblockchain` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn rescanblockchain(
    transport: &dyn TransportTrait,
    start_height: serde_json::Value,
    stop_height: serde_json::Value,
) -> Result<RescanblockchainResponse, TransportError> {
    let params = vec![json!(start_height), json!(stop_height)];
    let raw = transport.send_request("rescanblockchain", &params).await?;
    Ok(serde_json::from_value::<RescanblockchainResponse>(raw)?)
}
