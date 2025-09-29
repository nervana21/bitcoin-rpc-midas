//! Generated RPC response types
use serde::{Deserialize, Serialize};

/// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
/// Note: Use \"getwalletinfo\" to query the scanning progress.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AbortrescanResponse(pub bool);

/// Open an outbound connection to a specified node. This RPC is for testing only.
#[derive(Debug, Deserialize, Serialize)]
pub struct AddconnectionResponse {
    pub address: String,
    pub connection_type: String,
}

/// Add the address of a potential peer to an address manager table. This RPC is for testing only.
#[derive(Debug, Deserialize, Serialize)]
pub struct AddpeeraddressResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Analyzes and provides information about the current status of a PSBT and its inputs
#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyzepsbtResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_vsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_feerate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    pub next: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Bumps the fee of a transaction T, replacing it with a new transaction B.
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
pub struct BumpfeeResponse {
    pub txid: bitcoin::Txid,
    pub origfee: f64,
    pub fee: f64,
    pub errors: Vec<serde_json::Value>,
}

/// Combine multiple partially signed Bitcoin transactions into one transaction.
/// Implements the Combiner role.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CombinepsbtResponse(pub String);

/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CombinerawtransactionResponse(pub String);

/// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// createpsbt and walletcreatefundedpsbt should be used for new applications.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConverttopsbtResponse(pub String);

/// Creates a multi-signature address with n signatures of m keys required.
/// It returns a json object with the address and redeemScript.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatemultisigResponse {
    pub address: String,
    #[serde(rename = "redeemScript")]
    pub redeem_script: String,
    pub descriptor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}

/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreatepsbtResponse(pub String);

/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CreaterawtransactionResponse(pub String);

/// Creates and loads a new wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatewalletResponse {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}

/// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatewalletdescriptorResponse {
    pub descs: Vec<serde_json::Value>,
}

/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
#[derive(Debug, Deserialize, Serialize)]
pub struct DecodepsbtResponse {
    pub tx: serde_json::Value,
    pub global_xpubs: Vec<serde_json::Value>,
    pub psbt_version: u32,
    pub proprietary: Vec<serde_json::Value>,
    pub unknown: serde_json::Value,
    pub inputs: Vec<serde_json::Value>,
    pub outputs: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
}

/// Return a JSON object representing the serialized, hex-encoded transaction.
#[derive(Debug, Deserialize, Serialize)]
pub struct DecoderawtransactionResponse {
    pub txid: bitcoin::Txid,
    pub hash: String,
    pub size: u64,
    pub vsize: u64,
    pub weight: u64,
    pub version: u32,
    pub locktime: serde_json::Value,
    pub vin: Vec<serde_json::Value>,
    pub vout: Vec<serde_json::Value>,
}

/// Decode a hex-encoded script.
#[derive(Debug, Deserialize, Serialize)]
pub struct DecodescriptResponse {
    pub asm: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2sh: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segwit: Option<serde_json::Value>,
}

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

/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
#[derive(Debug, Deserialize, Serialize)]
pub struct DescriptorprocesspsbtResponse {
    pub psbt: String,
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
///
/// Unless the \"latest\" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
///
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
pub struct DumptxoutsetResponse {
    pub coins_written: u64,
    pub base_hash: String,
    pub base_height: u64,
    pub path: String,
    pub txoutset_hash: String,
    pub nchaintx: u64,
}

/// Simply echo back the input arguments. This command is for testing.
///
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
///
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EchoResponse(pub serde_json::Value);

/// Echo back the input argument, passing it through a spawned process in a multiprocess build.
/// This command is for testing.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EchoipcResponse(pub String);

/// Simply echo back the input arguments. This command is for testing.
///
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
///
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EchojsonResponse(pub serde_json::Value);

/// Encrypts the wallet with 'passphrase'. This is for first time encryption.
/// After this, any calls that interact with private keys such as sending or signing
/// will require the passphrase to be set prior to making these calls.
/// Use the walletpassphrase call for this, and then walletlock call.
/// If the wallet is already encrypted, use the walletpassphrasechange call.
/// ** IMPORTANT **
/// For security reasons, the encryption process will generate a new HD seed, resulting
/// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
/// securely back up the newly generated wallet file using the backupwallet RPC.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EncryptwalletResponse(pub String);

/// Returns a list of external signers from -signer.
#[derive(Debug, Deserialize, Serialize)]
pub struct EnumeratesignersResponse {
    pub signers: Vec<serde_json::Value>,
}

/// WARNING: This interface is unstable and may disappear or change!
///
/// WARNING: This is an advanced API call that is tightly coupled to the specific
/// implementation of fee estimation. The parameters it can be called with
/// and the results it returns will change if the internal implementation changes.
///
/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible. Uses virtual transaction size as
/// defined in BIP 141 (witness data is discounted).
#[derive(Debug, Deserialize, Serialize)]
pub struct EstimaterawfeeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long: Option<serde_json::Value>,
}

/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).
#[derive(Debug, Deserialize, Serialize)]
pub struct EstimatesmartfeeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feerate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<serde_json::Value>>,
    pub blocks: u64,
}

/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptWitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
#[derive(Debug, Deserialize, Serialize)]
pub struct FinalizepsbtResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    pub complete: bool,
}

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

/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateblockResponse {
    pub hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

/// Mine to a specified address and return the block hashes.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GeneratetoaddressResponse(pub Vec<serde_json::Value>);

/// Mine to a specified descriptor and return the block hashes.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GeneratetodescriptorResponse(pub Vec<serde_json::Value>);

/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetaddednodeinfoResponse(pub Vec<serde_json::Value>);

/// Returns the list of addresses assigned the specified label.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetaddressesbylabelResponse {
    pub address: serde_json::Value,
}

/// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetaddressinfoResponse {
    pub address: String,
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: String,
    pub ismine: bool,
    pub iswatchonly: bool,
    pub solvable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isscript: Option<bool>,
    pub ischange: bool,
    pub iswitness: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkeys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sigsrequired: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscompressed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdkeypath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdseedid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdmasterfingerprint: Option<String>,
    pub labels: Vec<serde_json::Value>,
}

/// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetaddrmaninfoResponse {
    pub network: serde_json::Value,
}

/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetbalanceResponse(pub f64);

/// Returns an object with all balances in BTC.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetbalancesResponse {
    pub mine: serde_json::Value,
    pub lastprocessedblock: serde_json::Value,
}

/// Returns the hash of the best (tip) block in the most-work fully-validated chain.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetbestblockhashResponse(pub bitcoin::BlockHash);

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetblockResponse {
    Raw(String),
    Verbose {
        hash: String,
        confirmations: u64,
        size: u64,
        strippedsize: u64,
        weight: u64,
        height: u64,
        version: u32,
        #[serde(rename = "versionHex")]
        version_hex: String,
        merkleroot: String,
        tx: Vec<serde_json::Value>,
        time: serde_json::Value,
        mediantime: serde_json::Value,
        nonce: u64,
        bits: String,
        target: String,
        difficulty: f64,
        chainwork: String,
        #[serde(rename = "nTx")]
        n_tx: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        previousblockhash: Option<bitcoin::BlockHash>,
        #[serde(skip_serializing_if = "Option::is_none")]
        nextblockhash: Option<bitcoin::BlockHash>,
    },
    Detailed {
        field_0: serde_json::Value,
        tx: Vec<serde_json::Value>,
    },
    Full {
        field_0: serde_json::Value,
        tx: Vec<serde_json::Value>,
    },
}

/// Returns an object containing various state info regarding blockchain processing.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetblockchaininfoResponse {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub bestblockhash: bitcoin::BlockHash,
    pub bits: String,
    pub target: String,
    pub difficulty: f64,
    pub time: serde_json::Value,
    pub mediantime: serde_json::Value,
    pub verificationprogress: f64,
    pub initialblockdownload: bool,
    pub chainwork: String,
    pub size_on_disk: u64,
    pub pruned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pruneheight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_pruning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune_target_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signet_challenge: Option<String>,
    pub warnings: Vec<serde_json::Value>,
}

/// Returns the height of the most-work fully-validated chain.
/// The genesis block has height 0.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetblockcountResponse(pub u64);

/// Retrieve a BIP 157 content filter for a particular block.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetblockfilterResponse {
    pub filter: String,
    pub header: String,
}

/// Attempt to fetch block from a given peer.
///
/// We must have the header for this block, e.g. using submitheader.
/// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
/// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
/// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
/// When a peer does not respond with a block, we will disconnect.
/// Note: The block could be re-pruned as soon as it is received.
///
/// Returns an empty JSON object if the request was successfully scheduled.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetblockfrompeerResponse(pub serde_json::Value);

/// Returns hash of block in best-block-chain at height provided.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetblockhashResponse(pub bitcoin::BlockHash);

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetblockheaderResponse {
    Verbose {
        hash: String,
        confirmations: u64,
        height: u64,
        version: u32,
        #[serde(rename = "versionHex")]
        version_hex: String,
        merkleroot: String,
        time: serde_json::Value,
        mediantime: serde_json::Value,
        nonce: u64,
        bits: String,
        target: String,
        difficulty: f64,
        chainwork: String,
        #[serde(rename = "nTx")]
        n_tx: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        previousblockhash: Option<bitcoin::BlockHash>,
        #[serde(skip_serializing_if = "Option::is_none")]
        nextblockhash: Option<bitcoin::BlockHash>,
    },
    Variant2(String),
}

/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won't work for some heights with pruning.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetblockstatsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgfee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgfeerate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgtxsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<bitcoin::BlockHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feerate_percentiles: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfeerate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxtxsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medianfee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediantime: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediantxsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minfee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minfeerate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mintxsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsidy: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtotal_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtotal_weight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtxs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_out: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_weight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalfee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase_actual: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc_actual: Option<u64>,
}

/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
/// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
/// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetblocktemplateResponse {
    Accepted(String),
    Variant3 {
        version: u32,
        rules: Vec<serde_json::Value>,
        vbavailable: serde_json::Value,
        capabilities: Vec<serde_json::Value>,
        vbrequired: u64,
        previousblockhash: bitcoin::BlockHash,
        transactions: Vec<serde_json::Value>,
        coinbaseaux: serde_json::Value,
        coinbasevalue: u64,
        longpollid: String,
        target: String,
        mintime: serde_json::Value,
        mutable: Vec<serde_json::Value>,
        noncerange: String,
        sigoplimit: u64,
        sizelimit: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        weightlimit: Option<u64>,
        curtime: serde_json::Value,
        bits: String,
        height: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        signet_challenge: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        default_witness_commitment: Option<String>,
    },
}

/// Return information about chainstates.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetchainstatesResponse {
    pub headers: u64,
    pub chainstates: Vec<serde_json::Value>,
}

/// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetchaintipsResponse(pub Vec<serde_json::Value>);

/// Compute statistics about the total number and rate of transactions in the chain.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetchaintxstatsResponse {
    pub time: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txcount: Option<u64>,
    pub window_final_block_hash: bitcoin::BlockHash,
    pub window_final_block_height: u64,
    pub window_block_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_interval: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_tx_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txrate: Option<f64>,
}

/// Returns the number of connections to other nodes.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetconnectioncountResponse(pub u64);

/// Returns an object containing various state info regarding deployments of consensus changes.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetdeploymentinfoResponse {
    pub hash: String,
    pub height: u64,
    pub deployments: serde_json::Value,
}

/// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
/// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
pub struct GetdescriptoractivityResponse {
    pub activity: Vec<serde_json::Value>,
}

/// Analyses a descriptor.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetdescriptorinfoResponse {
    pub descriptor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipath_expansion: Option<Vec<serde_json::Value>>,
    pub checksum: String,
    pub isrange: bool,
    pub issolvable: bool,
    pub hasprivatekeys: bool,
}

/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetdifficultyResponse(pub f64);

/// List all BIP 32 HD keys in the wallet and which descriptors use them.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GethdkeysResponse(pub Vec<serde_json::Value>);

/// Returns the status of one or all available indices currently running in the node.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetindexinfoResponse {
    pub name: serde_json::Value,
}

/// Returns an object containing information about memory usage.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetmemoryinfoResponse {
    Variant1(serde_json::Value),
    Variant2(String),
}

/// If txid is in the mempool, returns all in-mempool ancestors.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetmempoolancestorsResponse {
    Raw(Vec<String>),
    Verbose(serde_json::Value),
}

/// If txid is in the mempool, returns all in-mempool descendants.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetmempooldescendantsResponse {
    Raw(Vec<String>),
    Verbose(serde_json::Value),
}

/// Returns mempool data for given transaction
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmempoolentryResponse {
    pub vsize: u64,
    pub weight: u64,
    pub time: serde_json::Value,
    pub height: u64,
    pub descendantcount: u64,
    pub descendantsize: u64,
    pub ancestorcount: u64,
    pub ancestorsize: u64,
    pub wtxid: bitcoin::Txid,
    pub fees: serde_json::Value,
    pub depends: Vec<serde_json::Value>,
    pub spentby: Vec<serde_json::Value>,
    pub bip125_replaceable: bool,
    pub unbroadcast: bool,
}

/// Returns details on the active state of the TX memory pool.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmempoolinfoResponse {
    pub loaded: bool,
    pub size: u64,
    pub bytes: u64,
    pub usage: u64,
    pub total_fee: f64,
    pub maxmempool: u64,
    pub mempoolminfee: f64,
    pub minrelaytxfee: f64,
    pub incrementalrelayfee: f64,
    pub unbroadcastcount: u64,
    pub fullrbf: bool,
    pub permitbaremultisig: bool,
    pub maxdatacarriersize: u64,
}

/// Returns a json object containing mining-related information.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetmininginfoResponse {
    pub blocks: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentblockweight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentblocktx: Option<u64>,
    pub bits: String,
    pub difficulty: f64,
    pub target: String,
    pub networkhashps: u64,
    pub pooledtx: u64,
    pub blockmintxfee: f64,
    pub chain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signet_challenge: Option<String>,
    pub next: serde_json::Value,
    pub warnings: Vec<serde_json::Value>,
}

/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetnettotalsResponse {
    pub totalbytesrecv: u64,
    pub totalbytessent: u64,
    pub timemillis: serde_json::Value,
    pub uploadtarget: serde_json::Value,
}

/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetnetworkhashpsResponse(pub u64);

/// Returns an object containing various state info regarding P2P networking.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetnetworkinfoResponse {
    pub version: u32,
    pub subversion: String,
    pub protocolversion: u32,
    pub localservices: String,
    pub localservicesnames: Vec<serde_json::Value>,
    pub localrelay: bool,
    pub timeoffset: u64,
    pub connections: u64,
    pub connections_in: u64,
    pub connections_out: u64,
    pub networkactive: bool,
    pub networks: Vec<serde_json::Value>,
    pub relayfee: f64,
    pub incrementalfee: f64,
    pub localaddresses: Vec<String>,
    pub warnings: Vec<serde_json::Value>,
}

/// Returns a new Bitcoin address for receiving payments.
/// If 'label' is specified, it is added to the address book
/// so payments received with the address will be associated with 'label'.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetnewaddressResponse(pub String);

/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetnodeaddressesResponse(pub Vec<serde_json::Value>);

/// Shows transactions in the tx orphanage.
///
/// EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetorphantxsResponse {
    Variant1(Vec<bitcoin::Txid>),
    Variant2(Vec<serde_json::Value>),
    Variant3(Vec<serde_json::Value>),
}

/// Returns data about each connected network peer as a json array of objects.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetpeerinfoResponse(pub Vec<serde_json::Value>);

/// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetprioritisedtransactionsResponse {
    pub transactionid: serde_json::Value,
}

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Returns information on all address manager entries for the new and tried tables.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetrawaddrmanResponse {
    pub table: serde_json::Value,
}

/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetrawchangeaddressResponse(pub String);

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
///
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetrawmempoolResponse {
    Raw(Vec<String>),
    Verbose(serde_json::Value),
    RawWithSequence { txids: Vec<bitcoin::Txid>, mempool_sequence: u64 },
}

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
///
/// Hint: Use gettransaction for wallet transactions.
///
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetrawtransactionResponse {
    Raw(String),
    Verbose {
        #[serde(skip_serializing_if = "Option::is_none")]
        in_active_chain: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        blockhash: Option<bitcoin::BlockHash>,
        #[serde(skip_serializing_if = "Option::is_none")]
        confirmations: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        blocktime: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        time: Option<u64>,
        hex: String,
        txid: bitcoin::Txid,
        hash: String,
        size: u64,
        vsize: u64,
        weight: u64,
        version: u32,
        locktime: serde_json::Value,
        vin: Vec<serde_json::Value>,
        vout: Vec<serde_json::Value>,
    },
    Detailed {
        field_0: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        fee: Option<f64>,
        vin: Vec<serde_json::Value>,
    },
}

/// Returns the total amount received by the given address in transactions with at least minconf confirmations.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetreceivedbyaddressResponse(pub f64);

/// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetreceivedbylabelResponse(pub f64);

/// Returns details of the RPC server.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetrpcinfoResponse {
    pub active_commands: Vec<serde_json::Value>,
    pub logpath: String,
}

/// Get detailed information about in-wallet transaction <txid>
#[derive(Debug, Deserialize, Serialize)]
pub struct GettransactionResponse {
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    pub confirmations: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<bitcoin::BlockHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockheight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockindex: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocktime: Option<serde_json::Value>,
    pub txid: bitcoin::Txid,
    pub wtxid: bitcoin::Txid,
    pub walletconflicts: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by_txid: Option<bitcoin::Txid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_txid: Option<bitcoin::Txid>,
    pub mempoolconflicts: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    pub time: serde_json::Value,
    pub timereceived: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub bip125_replaceable: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_descs: Option<Vec<serde_json::Value>>,
    pub details: Vec<serde_json::Value>,
    pub hex: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded: Option<serde_json::Value>,
    pub lastprocessedblock: serde_json::Value,
}

/// Returns details about an unspent transaction output.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GettxoutResponse {
    Variant2 {
        bestblock: String,
        confirmations: u64,
        value: f64,
        #[serde(rename = "scriptPubKey")]
        script_pubkey: serde_json::Value,
        coinbase: bool,
    },
}

/// Returns a hex-encoded proof that \"txid\" was included in a block.
///
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GettxoutproofResponse(pub String);

/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
#[derive(Debug, Deserialize, Serialize)]
pub struct GettxoutsetinfoResponse {
    pub height: u64,
    pub bestblock: String,
    pub txouts: u64,
    pub bogosize: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_serialized_3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muhash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<u64>,
    pub total_amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_unspendable_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_info: Option<serde_json::Value>,
}

/// Scans the mempool to find transactions spending any of the given outputs
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GettxspendingprevoutResponse(pub Vec<serde_json::Value>);

/// Returns an object containing various wallet state info.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetwalletinfoResponse {
    pub walletname: String,
    pub walletversion: u32,
    pub format: String,
    pub txcount: u64,
    pub keypoolsize: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keypoolsize_hd_internal: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlocked_until: Option<serde_json::Value>,
    pub paytxfee: f64,
    pub private_keys_enabled: bool,
    pub avoid_reuse: bool,
    pub scanning: serde_json::Value,
    pub descriptors: bool,
    pub external_signer: bool,
    pub blank: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthtime: Option<serde_json::Value>,
    pub flags: Vec<serde_json::Value>,
    pub lastprocessedblock: serde_json::Value,
}

/// Returns information about the active ZeroMQ notifications.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetzmqnotificationsResponse(pub Vec<serde_json::Value>);

/// List all commands, or get help for a specified command.
#[derive(Debug, Deserialize, Serialize)]
pub struct HelpResponse {}

/// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
/// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second element will be imported as an internal descriptor.
///
/// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan is significantly faster if block filters are available (using startup option \"-blockfilterindex=1\").
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ImportdescriptorsResponse(pub Vec<serde_json::Value>);

/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ImportmempoolResponse(pub serde_json::Value);

/// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
/// No input in any of the PSBTs can be in more than one of the PSBTs.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct JoinpsbtsResponse(pub String);

/// Lists groups of addresses which have had their common ownership
/// made public by common use as inputs or as the resulting change
/// in past transactions
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListaddressgroupingsResponse(pub Vec<serde_json::Value>);

/// List all manually banned IPs/Subnets.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListbannedResponse(pub Vec<serde_json::Value>);

/// List all descriptors present in a wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListdescriptorsResponse {
    pub wallet_name: String,
    pub descriptors: Vec<serde_json::Value>,
}

/// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListlabelsResponse(pub Vec<serde_json::Value>);

/// Returns list of temporarily unspendable outputs.
/// See the lockunspent call to lock and unlock transactions for spending.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListlockunspentResponse(pub Vec<serde_json::Value>);

/// List balances by receiving address.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListreceivedbyaddressResponse(pub Vec<serde_json::Value>);

/// List received transactions by label.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListreceivedbylabelResponse(pub Vec<serde_json::Value>);

/// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
/// If \"blockhash\" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the \"removed\" array.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListsinceblockResponse {
    pub transactions: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<serde_json::Value>>,
    pub lastblock: String,
}

/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
///
/// Returns up to 'count' most recent transactions skipping the first 'from' transactions.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListtransactionsResponse(pub Vec<serde_json::Value>);

/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListunspentResponse(pub Vec<serde_json::Value>);

/// Returns a list of wallets in the wallet directory.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListwalletdirResponse {
    pub wallets: Vec<String>,
}

/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use \"getwalletinfo\"
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ListwalletsResponse(pub Vec<serde_json::Value>);

/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
///
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
///
/// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
#[derive(Debug, Deserialize, Serialize)]
pub struct LoadtxoutsetResponse {
    pub coins_loaded: u64,
    pub tip_hash: String,
    pub base_height: u64,
    pub path: String,
}

/// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct LoadwalletResponse {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}

/// Updates list of temporarily unspendable outputs.
/// Temporarily lock (unlock=false) or unlock (unlock=true) specified transaction outputs.
/// If no transaction outputs are specified when unlocking then all current locked transaction outputs are unlocked.
/// A locked transaction output will not be chosen by automatic coin selection, when spending bitcoins.
/// Manually selected coins are automatically unlocked.
/// Locks are stored in memory only, unless persistent=true, in which case they will be written to the
/// wallet database and loaded on node start. Unwritten (persistent=false) locks are always cleared
/// (by virtue of process exit) when a node stops or fails. Unlocking will clear both persistent and not.
/// Also see the listunspent call
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LockunspentResponse(pub bool);

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

/// Migrate the wallet to a descriptor wallet.
/// A new wallet backup will need to be made.
///
/// The migration process will create a backup of the wallet before migrating. This backup
/// file will be named <wallet name>-<timestamp>.legacy.bak and can be found in the directory
/// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
/// Encrypted wallets must have the passphrase provided as an argument to this call.
///
/// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
#[derive(Debug, Deserialize, Serialize)]
pub struct MigratewalletResponse {
    pub wallet_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchonly_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solvables_name: Option<String>,
    pub backup_path: String,
}

/// Accepts the transaction into mined blocks at a higher (or lower) priority
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PrioritisetransactionResponse(pub bool);

/// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
/// Requires `-prune` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via `getblockfrompeer`), local deletion is irreversible.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PruneblockchainResponse(pub u64);

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

/// Rescan the local blockchain for wallet related transactions.
/// Note: Use \"getwalletinfo\" to query the scanning progress.
/// The rescan is significantly faster if block filters are available
/// (using startup option \"-blockfilterindex=1\").
#[derive(Debug, Deserialize, Serialize)]
pub struct RescanblockchainResponse {
    pub start_height: u64,
    pub stop_height: u64,
}

/// Restores and loads a wallet from backup.
///
/// The rescan is significantly faster if block filters are available
/// (using startup option \"-blockfilterindex=1\").
#[derive(Debug, Deserialize, Serialize)]
pub struct RestorewalletResponse {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}

/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
#[derive(Debug, Deserialize, Serialize)]
pub struct SavemempoolResponse {
    pub filename: String,
}

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ScanblocksResponse {
    Started {
        from_height: u64,
        to_height: u64,
        relevant_blocks: Vec<serde_json::Value>,
        completed: bool,
    },
    Status {
        progress: u64,
        current_height: u64,
    },
    Aborted(bool),
}

/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
/// addr(<address>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
/// raw(<hex script>)                    Outputs whose output script equals the specified hex-encoded bytes
/// combo(<pubkey>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
/// pkh(<pubkey>)                        P2PKH outputs for the given pubkey
/// sh(multi(<n>,<pubkey>,<pubkey>,...)) P2SH-multisig outputs for the given threshold and pubkeys
/// tr(<pubkey>)                         P2TR
/// tr(<pubkey>,{pk(<pubkey>)})          P2TR with single fallback pubkey in tapscript
/// rawtr(<pubkey>)                      P2TR with the specified key as output key rather than inner
/// wsh(and_v(v:pk(<pubkey>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
///
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by \"/\", and optionally ending in \"/*\" (unhardened), or \"/*'\" or \"/*h\" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ScantxoutsetResponse {
    Started {
        success: bool,
        txouts: u64,
        height: u64,
        bestblock: String,
        unspents: Vec<serde_json::Value>,
        total_amount: f64,
    },
    Aborted(bool),
    Status(serde_json::Value),
}

/// Return RPC command JSON Schema descriptions.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SchemaResponse(pub serde_json::Value);

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Send a transaction.
#[derive(Debug, Deserialize, Serialize)]
pub struct SendResponse {
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
}

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
/// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
#[derive(Debug, Deserialize, Serialize)]
pub struct SendallResponse {
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
}

/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SendmanyResponse {
    Variant1(bitcoin::Txid),
    Variant2 { txid: bitcoin::Txid, fee_reason: String },
}

/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SendmsgtopeerResponse(pub serde_json::Value);

/// Submit a raw transaction (serialized, hex-encoded) to local node and network.
///
/// The transaction will be sent unconditionally to all peers, so using sendrawtransaction
/// for manual rebroadcast may degrade privacy by leaking the transaction's origin, as
/// nodes will normally not rebroadcast non-wallet transactions already in their mempool.
///
/// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
///
/// Related RPCs: createrawtransaction, signrawtransactionwithkey
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SendrawtransactionResponse(pub String);

/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SendtoaddressResponse {
    Variant1(bitcoin::Txid),
    Variant2 { txid: bitcoin::Txid, fee_reason: String },
}

/// Disable/enable all p2p network activity.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SetnetworkactiveResponse(pub bool);

/// (DEPRECATED) Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
/// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SettxfeeResponse(pub bool);

/// Change the state of the given wallet flag for a wallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct SetwalletflagResponse {
    pub flag_name: String,
    pub flag_state: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<String>,
}

/// Sign a message with the private key of an address
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignmessageResponse(pub String);

/// Sign a message with the private key of an address
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignmessagewithprivkeyResponse(pub String);

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignrawtransactionwithkeyResponse {
    pub hex: String,
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<serde_json::Value>>,
}

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignrawtransactionwithwalletResponse {
    pub hex: String,
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<serde_json::Value>>,
}

/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
#[derive(Debug, Deserialize, Serialize)]
pub struct SimulaterawtransactionResponse {
    pub balance_change: f64,
}

/// Request a graceful shutdown of Bitcoin Core.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StopResponse(pub String);

/// Attempts to submit new block to network.
/// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SubmitblockResponse {
    Variant2(String),
}

/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
#[derive(Debug, Deserialize, Serialize)]
pub struct SubmitpackageResponse {
    pub package_msg: String,
    pub tx_results: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_transactions: Option<Vec<serde_json::Value>>,
}

/// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
///
/// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
///
/// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
///
/// The maximum number of transactions allowed is 25.
///
/// This checks if transactions violate the consensus or policy rules.
///
/// See sendrawtransaction call.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TestmempoolacceptResponse(pub Vec<serde_json::Value>);

/// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
/// If both are specified, they must be identical.
#[derive(Debug, Deserialize, Serialize)]
pub struct UnloadwalletResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
}

/// Returns the total uptime of the server.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UptimeResponse(pub u64);

/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UtxoupdatepsbtResponse(pub String);

/// Return information about the given bitcoin address.
#[derive(Debug, Deserialize, Serialize)]
pub struct ValidateaddressResponse {
    pub isvalid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "scriptPubKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_pubkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isscript: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iswitness: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_locations: Option<Vec<serde_json::Value>>,
}

/// Verifies blockchain database.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VerifychainResponse(pub bool);

/// Verify a signed message.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VerifymessageResponse(pub bool);

/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VerifytxoutproofResponse(pub Vec<serde_json::Value>);

/// Waits for a specific new block and returns useful info about it.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
pub struct WaitforblockResponse {
    pub hash: String,
    pub height: u64,
}

/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
pub struct WaitforblockheightResponse {
    pub hash: String,
    pub height: u64,
}

/// Waits for any new block and returns useful info about it.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Deserialize, Serialize)]
pub struct WaitfornewblockResponse {
    pub hash: String,
    pub height: u64,
}

/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletcreatefundedpsbtResponse {
    pub psbt: String,
    pub fee: f64,
    pub changepos: u64,
}

/// Display address on an external signer for verification.
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletdisplayaddressResponse {
    pub address: String,
}

/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletprocesspsbtResponse {
    pub psbt: String,
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}
