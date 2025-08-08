//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Creates and loads a new wallet.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::createwallet;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.createwallet(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Creates and loads a new wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatewalletResponse {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}

/// Calls the `createwallet` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn createwallet(
    transport: &dyn TransportTrait,
    wallet_name: serde_json::Value,
    disable_private_keys: serde_json::Value,
    blank: serde_json::Value,
    passphrase: serde_json::Value,
    avoid_reuse: serde_json::Value,
    descriptors: serde_json::Value,
    load_on_startup: serde_json::Value,
    external_signer: serde_json::Value,
) -> Result<CreatewalletResponse, TransportError> {
    let params = vec![
        json!(wallet_name),
        json!(disable_private_keys),
        json!(blank),
        json!(passphrase),
        json!(avoid_reuse),
        json!(descriptors),
        json!(load_on_startup),
        json!(external_signer),
    ];
    let raw = transport.send_request("createwallet", &params).await?;
    Ok(serde_json::from_value::<CreatewalletResponse>(raw)?)
}
