//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getaddrmaninfo;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getaddrmaninfo().await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetaddrmaninfoResponse {
    pub network: serde_json::Value,
}

/// Calls the `getaddrmaninfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getaddrmaninfo(
    transport: &dyn TransportTrait,
) -> Result<GetaddrmaninfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getaddrmaninfo", &params).await?;
    Ok(serde_json::from_value::<GetaddrmaninfoResponse>(raw)?)
}
