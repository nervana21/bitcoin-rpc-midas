//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Imports keys from a wallet dump file (see dumpwallet). Requires a new wallet backup to include imported keys.
/// Note: Blockchain and Mempool will be rescanned after a successful import. Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::importwallet;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.importwallet(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};


/// Calls the `importwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn importwallet(transport: &dyn TransportTrait, filename: serde_json::Value) -> Result<Value, TransportError> {
    let params = vec![json!(filename)];
    let raw = transport.send_request("importwallet", &params).await?;
    Ok(raw)
}
