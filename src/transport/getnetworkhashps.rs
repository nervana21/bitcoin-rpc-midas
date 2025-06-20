//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getnetworkhashps;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getnetworkhashps(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Returns the estimated network hashes per second based on the last n blocks.
    /// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
    /// Pass in [height] to estimate the network speed at the time when a certain block was found.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetnetworkhashpsResponse(pub f64);



/// Calls the `getnetworkhashps` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getnetworkhashps(transport: &dyn TransportTrait, nblocks: serde_json::Value, height: serde_json::Value) -> Result<GetnetworkhashpsResponse, TransportError> {
    let params = vec![json!(nblocks), json!(height)];
    let raw = transport.send_request("getnetworkhashps", &params).await?;
    Ok(serde_json::from_value::<GetnetworkhashpsResponse>(raw)?)
}
