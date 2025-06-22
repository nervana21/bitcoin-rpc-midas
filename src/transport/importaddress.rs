//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Adds an address or script (in hex) that can be watched as if it were in your wallet but cannot be used to spend. Requires a new wallet backup.
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported address exists but related transactions are still missing, leading to temporarily incorrect/bogus balances and unspent outputs until rescan completes.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// If you have the full public key, you should call importpubkey instead of this.
/// Hint: use importmulti to import more than one address.
/// Note: If you import a non-standard raw script in hex form, outputs sending to it will be treated
/// as change, and not show up in many RPCs.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" for descriptor wallets.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::importaddress;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.importaddress(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `importaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn importaddress(transport: &dyn TransportTrait, address: serde_json::Value, label: serde_json::Value, rescan: serde_json::Value, p2sh: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(address), json!(label), json!(rescan), json!(p2sh)];
    let raw = transport.send_request("importaddress", &params).await?;
    Ok(raw)
}
