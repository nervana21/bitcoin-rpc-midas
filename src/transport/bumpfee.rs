//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Bumps the fee of an opt-in-RBF transaction T, replacing it with a new transaction B.
/// An opt-in RBF transaction with the given txid must be in the wallet.
/// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
/// It may add a new change output if one does not already exist.
/// All inputs in the original transaction will be included in the replacement transaction.
/// The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
/// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
/// The user can specify a confirmation target for estimatesmartfee.
/// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
/// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
/// returned by getnetworkinfo) to enter the node's mempool.
/// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::bumpfee;
///
/// let client = Client::new("http://127.0.0.1:8332", auth);
/// let result = client.bumpfee(/* params */).await?;
/// ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use transport::{TransportTrait, TransportError};
/// Bumps the fee of an opt-in-RBF transaction T, replacing it with a new transaction B.
    /// An opt-in RBF transaction with the given txid must be in the wallet.
    /// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
    /// It may add a new change output if one does not already exist.
    /// All inputs in the original transaction will be included in the replacement transaction.
    /// The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
    /// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
    /// The user can specify a confirmation target for estimatesmartfee.
    /// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
    /// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
    /// returned by getnetworkinfo) to enter the node's mempool.
    /// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *
#[derive(Debug, Deserialize, Serialize)]
pub struct BumpfeeResponse {
    pub txid: bitcoin::Txid,
    pub origfee: serde_json::Value,
    pub fee: serde_json::Value,
    pub errors: Vec<serde_json::Value>,
}



/// Calls the `bumpfee` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn bumpfee(transport: &dyn TransportTrait, txid: serde_json::Value, options: serde_json::Value) -> Result<BumpfeeResponse, TransportError> {
    let params = vec![json!(txid), json!(options)];
    let raw = transport.send_request("bumpfee", &params).await?;
    Ok(serde_json::from_value::<BumpfeeResponse>(raw)?)
}
