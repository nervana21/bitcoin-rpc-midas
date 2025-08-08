//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: latest

/// Gets and sets the logging configuration.
/// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
/// When called with arguments, adds or removes categories from debug logging and return the lists above.
/// The arguments are evaluated in order "include", "exclude".
/// If an item is both included and excluded, it will thus end up being excluded.
/// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
/// In addition, the following are available as category names with special meanings:
/// - "all",  "1" : represent all logging categories.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::latest::logging;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.logging(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Gets and sets the logging configuration.
/// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
/// When called with arguments, adds or removes categories from debug logging and return the lists above.
/// The arguments are evaluated in order \"include\", \"exclude\".
/// If an item is both included and excluded, it will thus end up being excluded.
/// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
/// In addition, the following are available as category names with special meanings:
/// - \"all\",  \"1\" : represent all logging categories.
#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingResponse {
    pub category: bool,
}

/// Calls the `logging` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn logging(
    transport: &dyn TransportTrait,
    include: serde_json::Value,
    exclude: serde_json::Value,
) -> Result<LoggingResponse, TransportError> {
    let params = vec![json!(include), json!(exclude)];
    let raw = transport.send_request("logging", &params).await?;
    Ok(serde_json::from_value::<LoggingResponse>(raw)?)
}
