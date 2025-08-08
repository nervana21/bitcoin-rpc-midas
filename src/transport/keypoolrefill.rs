//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Refills each descriptor keypool in the wallet up to the specified number of new keys.
/// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::keypoolrefill;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.keypoolrefill(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `keypoolrefill` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn keypoolrefill(transport: &dyn TransportTrait, newsize: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(newsize)];
    let raw = transport.send_request("keypoolrefill", &params).await?;
    Ok(raw)
}
