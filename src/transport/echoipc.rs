//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Echo back the input argument, passing it through a spawned process in a multiprocess build.
/// This command is for testing.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::echoipc;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.echoipc(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Echo back the input argument, passing it through a spawned process in a multiprocess build.
    /// This command is for testing.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EchoipcResponse(pub String);



/// Calls the `echoipc` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn echoipc(transport: &dyn TransportTrait, arg: serde_json::Value) -> Result<EchoipcResponse, TransportError> {
    let params = vec![json!(arg)];
    let raw = transport.send_request("echoipc", &params).await?;
    Ok(serde_json::from_value::<EchoipcResponse>(raw)?)
}
