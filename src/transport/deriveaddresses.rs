//! This file is auto-generated. Do not edit manually.
//! Generated for Bitcoin Core version: V29

/// Derives one or more addresses corresponding to an output descriptor.
/// Examples of output descriptors are:
/// pkh(<pubkey>)                                     P2PKH outputs for the given pubkey
/// wpkh(<pubkey>)                                    Native segwit P2PKH outputs for the given pubkey
/// sh(multi(<n>,<pubkey>,<pubkey>,...))              P2SH-multisig outputs for the given threshold and pubkeys
/// raw(<hex script>)                                 Outputs whose output script equals the specified hex-encoded bytes
/// tr(<pubkey>,multi_a(<n>,<pubkey>,<pubkey>,...))   P2TR-multisig outputs for the given threshold and pubkeys
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", where "h" represents a hardened child key.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.

/// # Example
/// ```rust
/// use bitcoin_rpc_codegen::client::V29::deriveaddresses;
///
/// let client = Client::new("http://127.0.0.1:18443", auth);
/// let result = client.deriveaddresses(/* params */).await?;
/// ```
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transport::{TransportError, TransportTrait};
/// Derives one or more addresses corresponding to an output descriptor.
/// Examples of output descriptors are:
/// pkh(<pubkey>)                                     P2PKH outputs for the given pubkey
/// wpkh(<pubkey>)                                    Native segwit P2PKH outputs for the given pubkey
/// sh(multi(<n>,<pubkey>,<pubkey>,...))              P2SH-multisig outputs for the given threshold and pubkeys
/// raw(<hex script>)                                 Outputs whose output script equals the specified hex-encoded bytes
/// tr(<pubkey>,multi_a(<n>,<pubkey>,<pubkey>,...))   P2TR-multisig outputs for the given threshold and pubkeys
///
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by \"/\", where \"h\" represents a hardened child key.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Debug, Deserialize, Serialize)]
pub struct DeriveaddressesResponse {}

/// Calls the `deriveaddresses` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn deriveaddresses(
    transport: &dyn TransportTrait,
    descriptor: serde_json::Value,
    range: serde_json::Value,
) -> Result<DeriveaddressesResponse, TransportError> {
    let params = vec![json!(descriptor), json!(range)];
    let raw = transport.send_request("deriveaddresses", &params).await?;
    Ok(serde_json::from_value::<DeriveaddressesResponse>(raw)?)
}
