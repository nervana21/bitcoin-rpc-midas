//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Lists groups of addresses which have had their common ownership
/// made public by common use as inputs or as the resulting change
/// in past transactions

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::listaddressgroupings;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.listaddressgroupings().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Lists groups of addresses which have had their common ownership
    /// made public by common use as inputs or as the resulting change
    /// in past transactions
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListaddressgroupingsResponse(pub Vec<serde_json::Value>);



/// Calls the `listaddressgroupings` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn listaddressgroupings(transport: &dyn TransportTrait) -> Result<ListaddressgroupingsResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listaddressgroupings", &params).await?;
    Ok(serde_json::from_value::<ListaddressgroupingsResponse>(raw)?)
}
