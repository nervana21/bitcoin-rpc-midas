//! This file is auto-generated. Do not edit manually.
//! Generated from Bitcoin Core v29.1

use serde::{Deserialize, Serialize};
use serde_json::json;
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
/// # Example: High-Level Client Usage (Recommended)
/// ```rust
/// use bitcoin_rpc_midas::*;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BitcoinTestClient::new().await?;
/// let result = client.deriveaddresses(/* params */).await?;
/// # Ok(())
/// # }
/// ```
/// # Example: Advanced - Direct Transport Function Usage
/// This approach is for advanced users who need direct control over the transport layer.
/// Most users should prefer the high-level client approach above.
/// ```rust
/// use bitcoin_rpc_midas::transport::deriveaddresses;
/// use bitcoin_rpc_midas::transport::{TransportTrait, DefaultTransport};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let transport = DefaultTransport::new(
///     "http://127.0.0.1:18443".to_string(),
///     Some(("rpcuser".to_string(), "rpcpassword".to_string()))
/// );
/// let result = deriveaddresses(&transport, /* params */).await?;
/// # Ok(())
/// # }
/// ```
#[allow(unused_imports)]
use serde_json::Value;

use crate::transport::{TransportError, TransportTrait};
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
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum DeriveaddressesResponse {
    Variant1(Vec<String>),
    Variant2(Vec<Vec<String>>),
}

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
