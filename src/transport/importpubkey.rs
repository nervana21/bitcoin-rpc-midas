//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Adds a public key (in hex) that can be watched as if it were in your wallet but cannot be used to spend. Requires a new wallet backup.
/// Hint: use importmulti to import more than one public key.
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported pubkey exists but related transactions are still missing, leading to temporarily incorrect/bogus balances and unspent outputs until rescan completes.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" with "combo(X)" for descriptor wallets.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::importpubkey;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.importpubkey(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `importpubkey` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn importpubkey(transport: &dyn TransportTrait, pubkey: serde_json::Value, label: serde_json::Value, rescan: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(pubkey), json!(label), json!(rescan)];
    let raw = transport.send_request("importpubkey", &params).await?;
    Ok(raw)
}
