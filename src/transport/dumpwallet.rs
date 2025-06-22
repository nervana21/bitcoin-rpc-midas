//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Dumps all wallet keys in a human-readable format to a server-side file. This does not allow overwriting existing files.
/// Imported scripts are included in the dumpfile, but corresponding BIP173 addresses, etc. may not be added automatically by importwallet.
/// Note that if your wallet contains keys which are not derived from your HD seed (e.g. imported keys), these are not covered by
/// only backing up the seed itself, and must be backed up too (e.g. ensure you back up the whole dumpfile).
/// Note: This command is only compatible with legacy wallets.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::dumpwallet;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.dumpwallet(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Dumps all wallet keys in a human-readable format to a server-side file. This does not allow overwriting existing files.
    /// Imported scripts are included in the dumpfile, but corresponding BIP173 addresses, etc. may not be added automatically by importwallet.
    /// Note that if your wallet contains keys which are not derived from your HD seed (e.g. imported keys), these are not covered by
    /// only backing up the seed itself, and must be backed up too (e.g. ensure you back up the whole dumpfile).
    /// Note: This command is only compatible with legacy wallets.
#[derive(Debug, Deserialize, Serialize)]
pub struct DumpwalletResponse {
    pub filename: String,
}



/// Calls the `dumpwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn dumpwallet(transport: &dyn TransportTrait, filename: serde_json::Value) -> Result<DumpwalletResponse, TransportError> {
    let params = vec![json!(filename)];
    let raw = transport.send_request("dumpwallet", &params).await?;
    Ok(serde_json::from_value::<DumpwalletResponse>(raw)?)
}
