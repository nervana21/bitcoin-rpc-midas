//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Shows transactions in the tx orphanage.
/// EXPERIMENTAL warning: this call may be changed in future releases.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getorphantxs;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getorphantxs(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Shows transactions in the tx orphanage.
    /// 
    /// EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetorphantxsResponse {
}



/// Calls the `getorphantxs` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getorphantxs(transport: &dyn TransportTrait, verbosity: serde_json::Value) -> Result<GetorphantxsResponse, TransportError> {
    let params = vec![json!(verbosity)];
    let raw = transport.send_request("getorphantxs", &params).await?;
    Ok(serde_json::from_value::<GetorphantxsResponse>(raw)?)
}
