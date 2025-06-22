//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// If the transaction has no inputs, they will be automatically selected to meet its out value.
/// It will add at most one change output to the outputs.
/// No existing outputs will be modified unless "subtractFeeFromOutputs" is specified.
/// Note that inputs which were signed may need to be resigned after completion since in/outputs have been added.
/// The inputs added will not be signed, use signrawtransactionwithkey
/// or signrawtransactionwithwallet for that.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
/// Note that all inputs selected must be of standard form and P2SH scripts must be
/// in the wallet using importaddress or addmultisigaddress (to calculate fees).
/// You can see whether this is the case by checking the "solvable" field in the listunspent output.
/// Only pay-to-pubkey, multisig, and P2SH versions thereof are currently supported for watch-only

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::fundrawtransaction;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.fundrawtransaction(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// If the transaction has no inputs, they will be automatically selected to meet its out value.
    /// It will add at most one change output to the outputs.
    /// No existing outputs will be modified unless \"subtractFeeFromOutputs\" is specified.
    /// Note that inputs which were signed may need to be resigned after completion since in/outputs have been added.
    /// The inputs added will not be signed, use signrawtransactionwithkey
    /// or signrawtransactionwithwallet for that.
    /// All existing inputs must either have their previous output transaction be in the wallet
    /// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
    /// Note that all inputs selected must be of standard form and P2SH scripts must be
    /// in the wallet using importaddress or addmultisigaddress (to calculate fees).
    /// You can see whether this is the case by checking the \"solvable\" field in the listunspent output.
    /// Only pay-to-pubkey, multisig, and P2SH versions thereof are currently supported for watch-only
#[derive(Debug, Deserialize, Serialize)]
pub struct FundrawtransactionResponse {
    pub hex: String,
    pub fee: f64,
    pub changepos: u64,
}



/// Calls the `fundrawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn fundrawtransaction(transport: &dyn TransportTrait, hexstring: serde_json::Value, options: serde_json::Value, iswitness: serde_json::Value) -> Result<FundrawtransactionResponse, TransportError> {
    let params = vec![json!(hexstring), json!(options), json!(iswitness)];
    let raw = transport.send_request("fundrawtransaction", &params).await?;
    Ok(serde_json::from_value::<FundrawtransactionResponse>(raw)?)
}
