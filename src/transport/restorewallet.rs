//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// Restores and loads a wallet from backup.
/// The rescan is significantly faster if block filters are available
/// (using startup option "-blockfilterindex=1").
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.restorewallet(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::restorewallet;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = restorewallet(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
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
    let params = vec![json!(wallet_name), json!(backup_file), json!(load_on_startup)];
    let raw = transport.send_request("restorewallet", &params).await?;
    Ok(serde_json::from_value::<RestorewalletResponse>(raw)?)
}
