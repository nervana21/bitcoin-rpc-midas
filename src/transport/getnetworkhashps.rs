//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::v29_1::getnetworkhashps;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.getnetworkhashps(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetnetworkhashpsResponse(pub u64);

/// Calls the `getnetworkhashps` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getnetworkhashps(
    transport: &dyn TransportTrait,
    nblocks: serde_json::Value,
    height: serde_json::Value,
) -> Result<GetnetworkhashpsResponse, TransportError> {
    let params = vec![json!(nblocks), json!(height)];
    let raw = transport.send_request("getnetworkhashps", &params).await?;
    Ok(serde_json::from_value::<GetnetworkhashpsResponse>(raw)?)
}
