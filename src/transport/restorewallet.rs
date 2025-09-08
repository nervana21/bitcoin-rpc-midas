//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Restores and loads a wallet from backup.
/// The rescan is significantly faster if block filters are available
/// (using startup option "-blockfilterindex=1").

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::restorewallet;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.restorewallet(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Restores and loads a wallet from backup.
///
/// The rescan is significantly faster if block filters are available
/// (using startup option \"-blockfilterindex=1\").
#[derive(Debug, Deserialize, Serialize)]
pub struct RestorewalletResponse {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}

/// Calls the `restorewallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn restorewallet(
    transport: &dyn TransportTrait,
    wallet_name: serde_json::Value,
    backup_file: serde_json::Value,
    load_on_startup: serde_json::Value,
) -> Result<RestorewalletResponse, TransportError> {
    let params = vec![
        json!(wallet_name),
        json!(backup_file),
        json!(load_on_startup),
    ];
    let raw = transport.send_request("restorewallet", &params).await?;
    Ok(serde_json::from_value::<RestorewalletResponse>(raw)?)
}
