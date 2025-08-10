//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// (DEPRECATED) Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
/// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::settxfee;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.settxfee(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// (DEPRECATED) Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
/// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SettxfeeResponse(pub bool);

/// Calls the `settxfee` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn settxfee(
    transport: &dyn TransportTrait,
    amount: serde_json::Value,
) -> Result<SettxfeeResponse, TransportError> {
    let params = vec![json!(amount)];
    let raw = transport.send_request("settxfee", &params).await?;
    Ok(serde_json::from_value::<SettxfeeResponse>(raw)?)
}
