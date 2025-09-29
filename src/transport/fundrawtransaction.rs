//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
/// If the transaction has no inputs, they will be automatically selected to meet its out value.
/// It will add at most one change output to the outputs.
/// No existing outputs will be modified unless "subtractFeeFromOutputs" is specified.
/// Note that inputs which were signed may need to be resigned after completion since in/outputs have been added.
/// The inputs added will not be signed, use signrawtransactionwithkey
/// or signrawtransactionwithwallet for that.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
/// Note that all inputs selected must be of standard form and P2SH scripts must be
/// in the wallet using importdescriptors (to calculate fees).
/// You can see whether this is the case by checking the "solvable" field in the listunspent output.
/// Note that if specifying an exact fee rate, the resulting transaction may have a higher fee rate
/// if the transaction has unconfirmed inputs. This is because the wallet will attempt to make the
/// entire package have the given fee rate, not the resulting transaction.
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.fundrawtransaction(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::fundrawtransaction;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = fundrawtransaction(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
/// If the transaction has no inputs, they will be automatically selected to meet its out value.
/// It will add at most one change output to the outputs.
/// No existing outputs will be modified unless \"subtractFeeFromOutputs\" is specified.
/// Note that inputs which were signed may need to be resigned after completion since in/outputs have been added.
/// The inputs added will not be signed, use signrawtransactionwithkey
/// or signrawtransactionwithwallet for that.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
/// Note that all inputs selected must be of standard form and P2SH scripts must be
/// in the wallet using importdescriptors (to calculate fees).
/// You can see whether this is the case by checking the \"solvable\" field in the listunspent output.
/// Note that if specifying an exact fee rate, the resulting transaction may have a higher fee rate
/// if the transaction has unconfirmed inputs. This is because the wallet will attempt to make the
/// entire package have the given fee rate, not the resulting transaction.
#[derive(Debug, Deserialize, Serialize)]
pub struct FundrawtransactionResponse {
    pub hex: String,
    pub fee: f64,
    pub changepos: u64,
}

/// Calls the `fundrawtransaction` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn fundrawtransaction(
    transport: &dyn TransportTrait,
    hexstring: serde_json::Value,
    options: serde_json::Value,
    iswitness: serde_json::Value,
) -> Result<FundrawtransactionResponse, TransportError> {
    let params = vec![json!(hexstring), json!(options), json!(iswitness)];
    let raw = transport.send_request("fundrawtransaction", &params).await?;
    Ok(serde_json::from_value::<FundrawtransactionResponse>(raw)?)
}
