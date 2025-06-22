//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// If txid is in the mempool, returns all in-mempool ancestors.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getmempoolancestors;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getmempoolancestors(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// If txid is in the mempool, returns all in-mempool ancestors.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmempoolancestorsResponse {
}



/// Calls the `getmempoolancestors` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getmempoolancestors(transport: &dyn TransportTrait, txid: serde_json::Value, verbose: serde_json::Value) -> Result<GetmempoolancestorsResponse, TransportError> {
    let params = vec![json!(txid), json!(verbose)];
    let raw = transport.send_request("getmempoolancestors", &params).await?;
    Ok(serde_json::from_value::<GetmempoolancestorsResponse>(raw)?)
}
