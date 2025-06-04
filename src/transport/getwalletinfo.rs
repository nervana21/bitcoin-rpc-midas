//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Returns an object containing various wallet state info.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::getwalletinfo;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.getwalletinfo().await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{Transport, TransportError};
/// Response for the `getwalletinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetwalletinfoResponse {
    /// the wallet name
    pub walletname: String,
    /// the wallet version
    pub walletversion: u64,
    /// the database format (bdb or sqlite)
    pub format: String,
    /// DEPRECATED. Identical to getbalances().mine.trusted
    pub balance: bitcoin::Amount,
    /// DEPRECATED. Identical to getbalances().mine.untrusted_pending
    pub unconfirmed_balance: bitcoin::Amount,
    /// DEPRECATED. Identical to getbalances().mine.immature
    pub immature_balance: bitcoin::Amount,
    /// the total number of transactions in the wallet
    pub txcount: u64,
    /// the UNIX epoch time of the oldest pre-generated key in the key pool. Legacy wallets only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keypoololdest: Option<serde_json::Value>,
    /// how many new keys are pre-generated (only counts external keys)
    pub keypoolsize: u64,
    /// how many new keys are pre-generated for internal use (used for change outputs, only appears if the wallet is using this feature, otherwise external keys are used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keypoolsize_hd_internal: Option<u64>,
    /// the UNIX epoch time until which the wallet is unlocked for transfers, or 0 if the wallet is locked (only present for passphrase-encrypted wallets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlocked_until: Option<serde_json::Value>,
    /// the transaction fee configuration, set in BTC/kvB
    pub paytxfee: bitcoin::Amount,
    /// the Hash160 of the HD seed (only present when HD is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdseedid: Option<String>,
    /// false if privatekeys are disabled for this wallet (enforced watch-only wallet)
    pub private_keys_enabled: bool,
    /// whether this wallet tracks clean/dirty coins in terms of reuse
    pub avoid_reuse: bool,
    /// current scanning details, or false if no scan is in progress
    pub scanning: serde_json::Value,
    /// whether this wallet uses descriptors for output script management
    pub descriptors: bool,
    /// whether this wallet is configured to use an external signer such as a hardware wallet
    pub external_signer: bool,
    /// Whether this wallet intentionally does not contain any keys, scripts, or descriptors
    pub blank: bool,
    /// The start time for blocks scanning. It could be modified by (re)importing any descriptor with an earlier timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthtime: Option<serde_json::Value>,
    /// hash and height of the block this information was generated on
    pub lastprocessedblock: bitcoin::Block,
}



/// Calls the `getwalletinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn getwalletinfo(transport: &dyn Transport) -> Result<GetwalletinfoResponse, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getwalletinfo", &params).await?;
    Ok(serde_json::from_value::<GetwalletinfoResponse>(raw)?)
}
