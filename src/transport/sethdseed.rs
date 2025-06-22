//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Set or generate a new HD wallet seed. Non-HD wallets will not be upgraded to being a HD wallet. Wallets that are already
/// HD will have a new HD seed set so that new keys added to the keypool will be derived from this new seed.
/// Note that you will need to MAKE A NEW BACKUP of your wallet after setting the HD wallet seed.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
/// Note: This command is only compatible with legacy wallets.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::sethdseed;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.sethdseed(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `sethdseed` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn sethdseed(transport: &dyn TransportTrait, newkeypool: serde_json::Value, seed: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(newkeypool), json!(seed)];
    let raw = transport.send_request("sethdseed", &params).await?;
    Ok(raw)
}
