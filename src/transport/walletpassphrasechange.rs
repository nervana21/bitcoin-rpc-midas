//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde_json::json;
/// Changes the wallet passphrase from 'oldpassphrase' to 'newpassphrase'.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.walletpassphrasechange(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::walletpassphrasechange;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = walletpassphrasechange(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};

/// Calls the `walletpassphrasechange` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn walletpassphrasechange(
    transport: &dyn TransportTrait,
    oldpassphrase: serde_json::Value,
    newpassphrase: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(oldpassphrase), json!(newpassphrase)];
    let raw = transport.send_request("walletpassphrasechange", &params).await?;
    Ok(raw)
}
