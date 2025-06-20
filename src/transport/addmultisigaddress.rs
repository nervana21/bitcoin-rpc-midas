//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Add an nrequired-to-sign multisignature address to the wallet. Requires a new wallet backup.
/// Each key is a Bitcoin address or hex-encoded public key.
/// This functionality is only intended for use with non-watchonly addresses.
/// See `importaddress` for watchonly p2sh address support.
/// If 'label' is specified, assign address to that label.
/// Note: This command is only compatible with legacy wallets.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::addmultisigaddress;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.addmultisigaddress(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Add an nrequired-to-sign multisignature address to the wallet. Requires a new wallet backup.
    /// Each key is a Bitcoin address or hex-encoded public key.
    /// This functionality is only intended for use with non-watchonly addresses.
    /// See `importaddress` for watchonly p2sh address support.
    /// If 'label' is specified, assign address to that label.
    /// Note: This command is only compatible with legacy wallets.
#[derive(Debug, Deserialize, Serialize)]
pub struct AddmultisigaddressResponse {
    pub address: String,
    pub redeem_script: String,
    pub descriptor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}



/// Calls the `addmultisigaddress` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn addmultisigaddress(transport: &dyn TransportTrait, nrequired: serde_json::Value, keys: serde_json::Value, label: serde_json::Value, address_type: serde_json::Value) -> Result<AddmultisigaddressResponse, TransportError> {
    let params = vec![json!(nrequired), json!(keys), json!(label), json!(address_type)];
    let raw = transport.send_request("addmultisigaddress", &params).await?;
    Ok(serde_json::from_value::<AddmultisigaddressResponse>(raw)?)
}
