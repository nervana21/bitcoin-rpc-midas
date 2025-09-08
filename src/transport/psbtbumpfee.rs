//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

/// Bumps the fee of a transaction T, replacing it with a new transaction B.
/// Returns a PSBT instead of creating and signing a new transaction.
/// A transaction with the given txid must be in the wallet.
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
/// use bitcoin_rpc_codegen::client::v29_1::psbtbumpfee;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.psbtbumpfee(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Bumps the fee of a transaction T, replacing it with a new transaction B.
/// Returns a PSBT instead of creating and signing a new transaction.
/// A transaction with the given txid must be in the wallet.
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
pub struct PsbtbumpfeeResponse {
    pub psbt: String,
    pub origfee: f64,
    pub fee: f64,
    pub errors: Vec<serde_json::Value>,
}

/// Calls the `psbtbumpfee` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn psbtbumpfee(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    options: serde_json::Value,
) -> Result<PsbtbumpfeeResponse, TransportError> {
    let params = vec![json!(txid), json!(options)];
    let raw = transport.send_request("psbtbumpfee", &params).await?;
    Ok(serde_json::from_value::<PsbtbumpfeeResponse>(raw)?)
}
