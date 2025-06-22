//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Import addresses/scripts (with private or public keys, redeem script (P2SH)), optionally rescanning the blockchain from the earliest creation time of the imported scripts. Requires a new wallet backup.
/// If an address/script is imported without all of the private keys required to spend from that address, it will be watchonly. The 'watchonly' option must be set to true in this case or a warning will be returned.
/// Conversely, if all the private keys are provided and the address/script is spendable, the watchonly option must be set to false, or a warning will be returned.
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" for descriptor wallets.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::importmulti;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.importmulti(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Import addresses/scripts (with private or public keys, redeem script (P2SH)), optionally rescanning the blockchain from the earliest creation time of the imported scripts. Requires a new wallet backup.
    /// If an address/script is imported without all of the private keys required to spend from that address, it will be watchonly. The 'watchonly' option must be set to true in this case or a warning will be returned.
    /// Conversely, if all the private keys are provided and the address/script is spendable, the watchonly option must be set to false, or a warning will be returned.
    /// 
    /// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
    /// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
    /// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
    /// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
    /// Note: Use \"getwalletinfo\" to query the scanning progress.
    /// Note: This command is only compatible with legacy wallets. Use \"importdescriptors\" for descriptor wallets.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ImportmultiResponse(pub Vec<serde_json::Value>);



/// Calls the `importmulti` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn importmulti(transport: &dyn TransportTrait, requests: serde_json::Value, options: serde_json::Value) -> Result<ImportmultiResponse, TransportError> {
    let params = vec![json!(requests), json!(options)];
    let raw = transport.send_request("importmulti", &params).await?;
    Ok(serde_json::from_value::<ImportmultiResponse>(raw)?)
}
