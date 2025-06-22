//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Simply echo back the input arguments. This command is for testing.
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::echojson;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.echojson(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Simply echo back the input arguments. This command is for testing.
    /// 
    /// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
    /// 
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EchojsonResponse(pub serde_json::Value);



/// Calls the `echojson` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn echojson(transport: &dyn TransportTrait, arg0: serde_json::Value, arg1: serde_json::Value, arg2: serde_json::Value, arg3: serde_json::Value, arg4: serde_json::Value, arg5: serde_json::Value, arg6: serde_json::Value, arg7: serde_json::Value, arg8: serde_json::Value, arg9: serde_json::Value) -> Result<EchojsonResponse, TransportError> {
    let params = vec![json!(arg0), json!(arg1), json!(arg2), json!(arg3), json!(arg4), json!(arg5), json!(arg6), json!(arg7), json!(arg8), json!(arg9)];
    let raw = transport.send_request("echojson", &params).await?;
    Ok(serde_json::from_value::<EchojsonResponse>(raw)?)
}
