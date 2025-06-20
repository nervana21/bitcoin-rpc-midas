//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns an object containing various state info regarding P2P networking.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getnetworkinfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getnetworkinfo().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns an object containing various state info regarding P2P networking.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetnetworkinfoResponse {
    pub version: u32,
    pub subversion: String,
    pub protocolversion: u32,
    pub localservices: String,
    pub localservicesnames: Vec<serde_json::Value>,
    pub localrelay: bool,
    pub timeoffset: u64,
    pub connections: f64,
    pub connections_in: f64,
    pub connections_out: f64,
    pub networkactive: bool,
    pub networks: Vec<serde_json::Value>,
    pub relayfee: f64,
    pub incrementalfee: f64,
    pub localaddresses: Vec<bitcoin::Address<bitcoin::address::NetworkUnchecked>>,
    pub warnings: Vec<serde_json::Value>,
}



/// Calls the `getnetworkinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getnetworkinfo(transport: &dyn TransportTrait) -> Result<GetnetworkinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getnetworkinfo", &params).await?;
    Ok(serde_json::from_value::<GetnetworkinfoResponse>(raw)?)
}
