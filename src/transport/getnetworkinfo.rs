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
use transport::{Transport, TransportError};
/// Response for the `getnetworkinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnetworkinfoResponse {
    /// the server version
    pub version: u64,
    /// the server subversion string
    pub subversion: String,
    /// the protocol version
    pub protocolversion: u64,
    /// the services we offer to the network
    pub localservices: String,
    /// the services we offer to the network, in human-readable form
    pub localservicesnames: Vec<serde_json::Value>,
    /// true if transaction relay is requested from peers
    pub localrelay: bool,
    /// the time offset
    pub timeoffset: u64,
    /// the total number of connections
    pub connections: u64,
    /// the number of inbound connections
    pub connections_in: u64,
    /// the number of outbound connections
    pub connections_out: u64,
    /// whether p2p networking is enabled
    pub networkactive: bool,
    /// information per network
    pub networks: Vec<serde_json::Value>,
    /// minimum relay fee rate for transactions in BTC/kvB
    pub relayfee: u64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    pub incrementalfee: u64,
    /// list of local addresses
    pub localaddresses: Vec<bitcoin::Address<bitcoin::address::NetworkUnchecked>>,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}



/// Calls the `getnetworkinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getnetworkinfo(transport: &dyn Transport) -> Result<GetnetworkinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getnetworkinfo", &params).await?;
    Ok(serde_json::from_value::<GetnetworkinfoResponse>(raw)?)
}
