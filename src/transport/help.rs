//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v30

use serde::{Deserialize, Serialize};
use serde_json::json;
/// List all commands, or get help for a specified command.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.help(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::help;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = help(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// List all commands, or get help for a specified command.
#[derive(Debug, Deserialize, Serialize)]
pub struct HelpResponse {}

/// Calls the `help` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn help(
    transport: &dyn TransportTrait,
    command: serde_json::Value,
) -> Result<HelpResponse, TransportError> {
    let params = vec![json!(command)];
    let raw = transport.send_request("help", &params).await?;
    Ok(serde_json::from_value::<HelpResponse>(raw)?)
}
