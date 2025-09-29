//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde_json::json;
/// Refills each descriptor keypool in the wallet up to the specified number of new keys.
/// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.keypoolrefill(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::keypoolrefill;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = keypoolrefill(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};

/// Calls the `keypoolrefill` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn keypoolrefill(
    transport: &dyn TransportTrait,
    newsize: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(newsize)];
    let raw = transport.send_request("keypoolrefill", &params).await?;
    Ok(raw)
}
