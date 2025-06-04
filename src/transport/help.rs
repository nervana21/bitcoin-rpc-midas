//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// List all commands, or get help for a specified command.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::help;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.help(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `help` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct HelpResponse {
    /// The help text
    pub field_0: String,
    pub field_1: serde_json::Value,
}



/// Calls the `help` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn help(transport: &dyn Transport, command: serde_json::Value) -> Result<HelpResponse, TransportError> {
    let params = vec![json!(command)];
    let raw = transport.send_request("help", &params).await?;
    Ok(serde_json::from_value::<HelpResponse>(raw)?)
}
