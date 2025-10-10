//! Parameter structs for RPC method calls
use bitcoin_rpc_types::HashOrHeight;
use serde::Serialize;

/// Mark in-wallet transaction <txid> as abandoned
/// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
/// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
/// It only works on transactions which are not included in a block and are not currently in the mempool.
/// It has no effect on transactions which are already abandoned.
#[derive(Debug, Serialize)]
pub struct AbandontransactionParams {
    pub txid: bitcoin::Txid,
}

/// Open an outbound connection to a specified node. This RPC is for testing only.
#[derive(Debug, Serialize)]
pub struct AddconnectionParams {
    pub address: String,
    pub connection_type: String,
    pub v2transport: bool,
}

/// Attempts to add or remove a node from the addnode list.
/// Or try a connection to a node once.
/// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
/// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
/// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
#[derive(Debug, Serialize)]
pub struct AddnodeParams {
    pub node: String,
    pub command: String,
    pub v2transport: bool,
}

/// Add the address of a potential peer to an address manager table. This RPC is for testing only.
#[derive(Debug, Serialize)]
pub struct AddpeeraddressParams {
    pub address: String,
    pub port: u16,
    pub tried: bool,
}

/// Analyzes and provides information about the current status of a PSBT and its inputs
#[derive(Debug, Serialize)]
pub struct AnalyzepsbtParams {
    pub psbt: String,
}

/// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
#[derive(Debug, Serialize)]
pub struct BackupwalletParams {
    pub destination: String,
}

/// Bumps the fee of a transaction T, replacing it with a new transaction B.
/// A transaction with the given txid must be in the wallet.
/// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
/// It may add a new change output if one does not already exist.
/// All inputs in the original transaction will be included in the replacement transaction.
/// The command will fail if the wallet or mempool contains a transaction that spends one of T"s outputs.
/// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
/// The user can specify a confirmation target for estimatesmartfee.
/// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
/// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
/// returned by getnetworkinfo) to enter the node"s mempool.
/// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *
#[derive(Debug, Serialize)]
pub struct BumpfeeParams {
    pub txid: bitcoin::Txid,
    pub options: serde_json::Value,
}

/// Combine multiple partially signed Bitcoin transactions into one transaction.
/// Implements the Combiner role.
#[derive(Debug, Serialize)]
pub struct CombinepsbtParams {
    pub txs: Vec<serde_json::Value>,
}

/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.
#[derive(Debug, Serialize)]
pub struct CombinerawtransactionParams {
    pub txs: Vec<serde_json::Value>,
}

/// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// createpsbt and walletcreatefundedpsbt should be used for new applications.
#[derive(Debug, Serialize)]
pub struct ConverttopsbtParams {
    pub hexstring: String,
    pub permitsigdata: bool,
    pub iswitness: bool,
}

/// Creates a multi-signature address with n signatures of m keys required.
/// It returns a json object with the address and redeemScript.
#[derive(Debug, Serialize)]
pub struct CreatemultisigParams {
    pub nrequired: u32,
    pub keys: Vec<String>,
    pub address_type: String,
}

/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
/// Note that the transaction"s inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[derive(Debug, Serialize)]
pub struct CreatepsbtParams {
    pub inputs: Vec<serde_json::Value>,
    pub outputs: Vec<serde_json::Value>,
    pub locktime: u32,
    pub replaceable: bool,
    pub version: u32,
}

/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction"s inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[derive(Debug, Serialize)]
pub struct CreaterawtransactionParams {
    pub inputs: Vec<serde_json::Value>,
    pub outputs: Vec<serde_json::Value>,
    pub locktime: u32,
    pub replaceable: bool,
    pub version: u32,
}

/// Creates and loads a new wallet.
#[derive(Debug, Serialize)]
pub struct CreatewalletParams {
    pub wallet_name: String,
    pub disable_private_keys: bool,
    pub blank: bool,
    pub passphrase: String,
    pub avoid_reuse: bool,
    pub descriptors: bool,
    pub load_on_startup: bool,
    pub external_signer: bool,
}

/// Creates the wallet"s descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct CreatewalletdescriptorParams {
    pub _type: String,
    pub options: serde_json::Value,
}

/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
#[derive(Debug, Serialize)]
pub struct DecodepsbtParams {
    pub psbt: String,
}

/// Return a JSON object representing the serialized, hex-encoded transaction.
#[derive(Debug, Serialize)]
pub struct DecoderawtransactionParams {
    pub hexstring: String,
    pub iswitness: bool,
}

/// Decode a hex-encoded script.
#[derive(Debug, Serialize)]
pub struct DecodescriptParams {
    pub hexstring: String,
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
/// or more path elements separated by "/", where "h" represents a hardened child key.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Debug, Serialize)]
pub struct DeriveaddressesParams {
    pub descriptor: String,
    pub range: serde_json::Value,
}

/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
#[derive(Debug, Serialize)]
pub struct DescriptorprocesspsbtParams {
    pub psbt: String,
    pub descriptors: Vec<serde_json::Value>,
    pub sighashtype: String,
    pub bip32derivs: bool,
    pub finalize: bool,
}

/// Immediately disconnects from the specified peer node.
///
/// Strictly one out of "address" and "nodeid" can be provided to identify the node.
///
/// To disconnect by nodeid, either set "address" to the empty string, or call using the named "nodeid" argument only.
#[derive(Debug, Serialize)]
pub struct DisconnectnodeParams {
    pub address: String,
    pub nodeid: u64,
}

/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
///
/// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
///
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Serialize)]
pub struct DumptxoutsetParams {
    pub path: String,
    pub _type: String,
    pub options: serde_json::Value,
}

/// Simply echo back the input arguments. This command is for testing.
///
/// It will return an internal bug report when arg9="trigger_internal_bug" is passed.
///
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Debug, Serialize)]
pub struct EchoParams {
    pub arg0: String,
    pub arg1: String,
    pub arg2: String,
    pub arg3: String,
    pub arg4: String,
    pub arg5: String,
    pub arg6: String,
    pub arg7: String,
    pub arg8: String,
    pub arg9: String,
}

/// Echo back the input argument, passing it through a spawned process in a multiprocess build.
/// This command is for testing.
#[derive(Debug, Serialize)]
pub struct EchoipcParams {
    pub arg: String,
}

/// Simply echo back the input arguments. This command is for testing.
///
/// It will return an internal bug report when arg9="trigger_internal_bug" is passed.
///
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Debug, Serialize)]
pub struct EchojsonParams {
    pub arg0: String,
    pub arg1: String,
    pub arg2: String,
    pub arg3: String,
    pub arg4: String,
    pub arg5: String,
    pub arg6: String,
    pub arg7: String,
    pub arg8: String,
    pub arg9: String,
}

/// Encrypts the wallet with "passphrase". This is for first time encryption.
/// After this, any calls that interact with private keys such as sending or signing
/// will require the passphrase to be set prior to making these calls.
/// Use the walletpassphrase call for this, and then walletlock call.
/// If the wallet is already encrypted, use the walletpassphrasechange call.
/// ** IMPORTANT **
/// For security reasons, the encryption process will generate a new HD seed, resulting
/// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
/// securely back up the newly generated wallet file using the backupwallet RPC.
#[derive(Debug, Serialize)]
pub struct EncryptwalletParams {
    pub passphrase: String,
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
#[derive(Debug, Serialize)]
pub struct EstimaterawfeeParams {
    pub conf_target: u64,
    pub threshold: u64,
}

/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).
#[derive(Debug, Serialize)]
pub struct EstimatesmartfeeParams {
    pub conf_target: u64,
    pub estimate_mode: String,
}

/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
#[derive(Debug, Serialize)]
pub struct FinalizepsbtParams {
    pub psbt: String,
    pub extract: bool,
}

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
#[derive(Debug, Serialize)]
pub struct FundrawtransactionParams {
    pub hexstring: String,
    pub options: serde_json::Value,
    pub iswitness: bool,
}

/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
#[derive(Debug, Serialize)]
pub struct GenerateblockParams {
    pub output: String,
    pub transactions: Vec<serde_json::Value>,
    pub submit: bool,
}

/// Mine to a specified address and return the block hashes.
#[derive(Debug, Serialize)]
pub struct GeneratetoaddressParams {
    pub nblocks: u64,
    pub address: String,
    pub maxtries: u64,
}

/// Mine to a specified descriptor and return the block hashes.
#[derive(Debug, Serialize)]
pub struct GeneratetodescriptorParams {
    pub num_blocks: u64,
    pub descriptor: String,
    pub maxtries: u64,
}

/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
#[derive(Debug, Serialize)]
pub struct GetaddednodeinfoParams {
    pub node: String,
}

/// Returns the list of addresses assigned the specified label.
#[derive(Debug, Serialize)]
pub struct GetaddressesbylabelParams {
    pub label: String,
}

/// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
#[derive(Debug, Serialize)]
pub struct GetaddressinfoParams {
    pub address: String,
}

/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
#[derive(Debug, Serialize)]
pub struct GetbalanceParams {
    pub dummy: Option<String>,
    pub minconf: u32,
    pub include_watchonly: bool,
    pub avoid_reuse: bool,
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block "hash".
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Debug, Serialize)]
pub struct GetblockParams {
    pub blockhash: bitcoin::BlockHash,
    pub verbosity: u32,
}

/// Retrieve a BIP 157 content filter for a particular block.
#[derive(Debug, Serialize)]
pub struct GetblockfilterParams {
    pub blockhash: bitcoin::BlockHash,
    pub filtertype: String,
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
#[derive(Debug, Serialize)]
pub struct GetblockfrompeerParams {
    pub blockhash: bitcoin::BlockHash,
    pub peer_id: u64,
}

/// Returns hash of block in best-block-chain at height provided.
#[derive(Debug, Serialize)]
pub struct GetblockhashParams {
    pub height: u64,
}

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader "hash".
/// If verbose is true, returns an Object with information about blockheader <hash>.
#[derive(Debug, Serialize)]
pub struct GetblockheaderParams {
    pub blockhash: bitcoin::BlockHash,
    pub verbose: bool,
}

/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won"t work for some heights with pruning.
#[derive(Debug, Serialize)]
pub struct GetblockstatsParams {
    pub hash_or_height: HashOrHeight,
    pub stats: Vec<String>,
}

/// If the request parameters include a "mode" key, that is used to explicitly select between the default "template" request or a "proposal".
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
/// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
/// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
#[derive(Debug, Serialize)]
pub struct GetblocktemplateParams {
    pub template_request: serde_json::Value,
}

/// Compute statistics about the total number and rate of transactions in the chain.
#[derive(Debug, Serialize)]
pub struct GetchaintxstatsParams {
    pub nblocks: u64,
    pub blockhash: bitcoin::BlockHash,
}

/// Returns an object containing various state info regarding deployments of consensus changes.
#[derive(Debug, Serialize)]
pub struct GetdeploymentinfoParams {
    pub blockhash: bitcoin::BlockHash,
}

/// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the ``relevant_blocks`` output of ``scanblocks()``.
/// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Serialize)]
pub struct GetdescriptoractivityParams {
    pub blockhashes: Vec<serde_json::Value>,
    pub scanobjects: Vec<serde_json::Value>,
    pub include_mempool: bool,
}

/// Analyses a descriptor.
#[derive(Debug, Serialize)]
pub struct GetdescriptorinfoParams {
    pub descriptor: String,
}

/// List all BIP 32 HD keys in the wallet and which descriptors use them.
#[derive(Debug, Serialize)]
pub struct GethdkeysParams {
    pub options: serde_json::Value,
}

/// Returns the status of one or all available indices currently running in the node.
#[derive(Debug, Serialize)]
pub struct GetindexinfoParams {
    pub index_name: String,
}

/// Returns an object containing information about memory usage.
#[derive(Debug, Serialize)]
pub struct GetmemoryinfoParams {
    pub mode: String,
}

/// If txid is in the mempool, returns all in-mempool ancestors.
#[derive(Debug, Serialize)]
pub struct GetmempoolancestorsParams {
    pub txid: bitcoin::Txid,
    pub verbose: bool,
}

/// If txid is in the mempool, returns all in-mempool descendants.
#[derive(Debug, Serialize)]
pub struct GetmempooldescendantsParams {
    pub txid: bitcoin::Txid,
    pub verbose: bool,
}

/// Returns mempool data for given transaction
#[derive(Debug, Serialize)]
pub struct GetmempoolentryParams {
    pub txid: bitcoin::Txid,
}

/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.
#[derive(Debug, Serialize)]
pub struct GetnetworkhashpsParams {
    pub nblocks: u64,
    pub height: u64,
}

/// Returns a new Bitcoin address for receiving payments.
/// If "label" is specified, it is added to the address book
/// so payments received with the address will be associated with "label".
#[derive(Debug, Serialize)]
pub struct GetnewaddressParams {
    pub label: String,
    pub address_type: String,
}

/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.
#[derive(Debug, Serialize)]
pub struct GetnodeaddressesParams {
    pub count: u64,
    pub network: String,
}

/// Shows transactions in the tx orphanage.
///
/// EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Debug, Serialize)]
pub struct GetorphantxsParams {
    pub verbosity: u32,
}

/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
#[derive(Debug, Serialize)]
pub struct GetrawchangeaddressParams {
    pub address_type: String,
}

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
///
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Debug, Serialize)]
pub struct GetrawmempoolParams {
    pub verbose: bool,
    pub mempool_sequence: bool,
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
#[derive(Debug, Serialize)]
pub struct GetrawtransactionParams {
    pub txid: bitcoin::Txid,
    pub verbosity: u32,
    pub blockhash: bitcoin::BlockHash,
}

/// Returns the total amount received by the given address in transactions with at least minconf confirmations.
#[derive(Debug, Serialize)]
pub struct GetreceivedbyaddressParams {
    pub address: String,
    pub minconf: u32,
    pub include_immature_coinbase: bool,
}

/// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
#[derive(Debug, Serialize)]
pub struct GetreceivedbylabelParams {
    pub label: String,
    pub minconf: u32,
    pub include_immature_coinbase: bool,
}

/// Get detailed information about in-wallet transaction <txid>
#[derive(Debug, Serialize)]
pub struct GettransactionParams {
    pub txid: bitcoin::Txid,
    pub include_watchonly: bool,
    pub verbose: bool,
}

/// Returns details about an unspent transaction output.
#[derive(Debug, Serialize)]
pub struct GettxoutParams {
    pub txid: bitcoin::Txid,
    pub n: u32,
    pub include_mempool: bool,
}

/// Returns a hex-encoded proof that "txid" was included in a block.
///
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
#[derive(Debug, Serialize)]
pub struct GettxoutproofParams {
    pub txids: Vec<bitcoin::Txid>,
    pub blockhash: bitcoin::BlockHash,
}

/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
#[derive(Debug, Serialize)]
pub struct GettxoutsetinfoParams {
    pub hash_type: String,
    pub hash_or_height: HashOrHeight,
    pub use_index: bool,
}

/// Scans the mempool to find transactions spending any of the given outputs
#[derive(Debug, Serialize)]
pub struct GettxspendingprevoutParams {
    pub outputs: Vec<serde_json::Value>,
}

/// List all commands, or get help for a specified command.
#[derive(Debug, Serialize)]
pub struct HelpParams {
    pub command: String,
}

/// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
/// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second element will be imported as an internal descriptor.
///
/// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
#[derive(Debug, Serialize)]
pub struct ImportdescriptorsParams {
    pub requests: Vec<serde_json::Value>,
}

/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
#[derive(Debug, Serialize)]
pub struct ImportmempoolParams {
    pub filepath: String,
    pub options: serde_json::Value,
}

/// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
#[derive(Debug, Serialize)]
pub struct ImportprunedfundsParams {
    pub rawtransaction: String,
    pub txoutproof: String,
}

/// Permanently marks a block as invalid, as if it violated a consensus rule.
#[derive(Debug, Serialize)]
pub struct InvalidateblockParams {
    pub blockhash: bitcoin::BlockHash,
}

/// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
/// No input in any of the PSBTs can be in more than one of the PSBTs.
#[derive(Debug, Serialize)]
pub struct JoinpsbtsParams {
    pub txs: Vec<serde_json::Value>,
}

/// Refills each descriptor keypool in the wallet up to the specified number of new keys.
/// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
///
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct KeypoolrefillParams {
    pub newsize: u64,
}

/// List all descriptors present in a wallet.
#[derive(Debug, Serialize)]
pub struct ListdescriptorsParams {
    pub private: bool,
}

/// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
#[derive(Debug, Serialize)]
pub struct ListlabelsParams {
    pub purpose: String,
}

/// List balances by receiving address.
#[derive(Debug, Serialize)]
pub struct ListreceivedbyaddressParams {
    pub minconf: u32,
    pub include_empty: bool,
    pub include_watchonly: bool,
    pub address_filter: String,
    pub include_immature_coinbase: bool,
}

/// List received transactions by label.
#[derive(Debug, Serialize)]
pub struct ListreceivedbylabelParams {
    pub minconf: u32,
    pub include_empty: bool,
    pub include_watchonly: bool,
    pub include_immature_coinbase: bool,
}

/// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
/// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
#[derive(Debug, Serialize)]
pub struct ListsinceblockParams {
    pub blockhash: bitcoin::BlockHash,
    pub target_confirmations: u64,
    pub include_watchonly: bool,
    pub include_removed: bool,
    pub include_change: bool,
    pub label: String,
}

/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
///
/// Returns up to "count" most recent transactions skipping the first "from" transactions.
#[derive(Debug, Serialize)]
pub struct ListtransactionsParams {
    pub label: String,
    pub count: u64,
    pub skip: u64,
    pub include_watchonly: bool,
}

/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
#[derive(Debug, Serialize)]
pub struct ListunspentParams {
    pub minconf: u32,
    pub maxconf: u32,
    pub addresses: Vec<String>,
    pub include_unsafe: bool,
    pub query_options: serde_json::Value,
}

/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network"s tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
///
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
///
/// You can find more information on this process in the ``assumeutxo`` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
#[derive(Debug, Serialize)]
pub struct LoadtxoutsetParams {
    pub path: String,
}

/// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.
#[derive(Debug, Serialize)]
pub struct LoadwalletParams {
    pub filename: String,
    pub load_on_startup: bool,
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
#[derive(Debug, Serialize)]
pub struct LockunspentParams {
    pub unlock: bool,
    pub transactions: Vec<serde_json::Value>,
    pub persistent: bool,
}

/// Gets and sets the logging configuration.
/// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
/// When called with arguments, adds or removes categories from debug logging and return the lists above.
/// The arguments are evaluated in order "include", "exclude".
/// If an item is both included and excluded, it will thus end up being excluded.
/// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
/// In addition, the following are available as category names with special meanings:
/// - "all",  "1" : represent all logging categories.
#[derive(Debug, Serialize)]
pub struct LoggingParams {
    pub include: Vec<serde_json::Value>,
    pub exclude: Vec<serde_json::Value>,
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
#[derive(Debug, Serialize)]
pub struct MigratewalletParams {
    pub wallet_name: String,
    pub passphrase: String,
}

/// Bump the scheduler into the future (-regtest only)
#[derive(Debug, Serialize)]
pub struct MockschedulerParams {
    pub delta_time: u64,
}

/// Treats a block as if it were received before others with the same work.
///
/// A later preciousblock call can override the effect of an earlier one.
///
/// The effects of preciousblock are not retained across restarts.
#[derive(Debug, Serialize)]
pub struct PreciousblockParams {
    pub blockhash: bitcoin::BlockHash,
}

/// Accepts the transaction into mined blocks at a higher (or lower) priority
#[derive(Debug, Serialize)]
pub struct PrioritisetransactionParams {
    pub txid: bitcoin::Txid,
    pub dummy: Option<String>,
    pub fee_delta: f64,
}

/// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
/// Requires ``-prune`` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via ``getblockfrompeer``), local deletion is irreversible.
#[derive(Debug, Serialize)]
pub struct PruneblockchainParams {
    pub height: u64,
}

/// Bumps the fee of a transaction T, replacing it with a new transaction B.
/// Returns a PSBT instead of creating and signing a new transaction.
/// A transaction with the given txid must be in the wallet.
/// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
/// It may add a new change output if one does not already exist.
/// All inputs in the original transaction will be included in the replacement transaction.
/// The command will fail if the wallet or mempool contains a transaction that spends one of T"s outputs.
/// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
/// The user can specify a confirmation target for estimatesmartfee.
/// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
/// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
/// returned by getnetworkinfo) to enter the node"s mempool.
/// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *
#[derive(Debug, Serialize)]
pub struct PsbtbumpfeeParams {
    pub txid: bitcoin::Txid,
    pub options: serde_json::Value,
}

/// Removes invalidity status of a block, its ancestors and its descendants, reconsider them for activation.
/// This can be used to undo the effects of invalidateblock.
#[derive(Debug, Serialize)]
pub struct ReconsiderblockParams {
    pub blockhash: bitcoin::BlockHash,
}

/// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
#[derive(Debug, Serialize)]
pub struct RemoveprunedfundsParams {
    pub txid: bitcoin::Txid,
}

/// Rescan the local blockchain for wallet related transactions.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// The rescan is significantly faster if block filters are available
/// (using startup option "-blockfilterindex=1").
#[derive(Debug, Serialize)]
pub struct RescanblockchainParams {
    pub start_height: u64,
    pub stop_height: u64,
}

/// Restores and loads a wallet from backup.
///
/// The rescan is significantly faster if block filters are available
/// (using startup option "-blockfilterindex=1").
#[derive(Debug, Serialize)]
pub struct RestorewalletParams {
    pub wallet_name: String,
    pub backup_file: String,
    pub load_on_startup: bool,
}

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Serialize)]
pub struct ScanblocksParams {
    pub action: String,
    pub scanobjects: Vec<serde_json::Value>,
    pub start_height: u64,
    pub stop_height: u64,
    pub filtertype: String,
    pub options: serde_json::Value,
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
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*"" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Debug, Serialize)]
pub struct ScantxoutsetParams {
    pub action: String,
    pub scanobjects: Vec<serde_json::Value>,
}

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Send a transaction.
#[derive(Debug, Serialize)]
pub struct SendParams {
    pub outputs: Vec<serde_json::Value>,
    pub conf_target: u64,
    pub estimate_mode: String,
    pub fee_rate: f64,
    pub options: serde_json::Value,
    pub version: u32,
}

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
/// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using ``send_max`` to exclude inputs that are worth less than the fees needed to spend them.
#[derive(Debug, Serialize)]
pub struct SendallParams {
    pub recipients: Vec<serde_json::Value>,
    pub conf_target: u64,
    pub estimate_mode: String,
    pub fee_rate: f64,
    pub options: serde_json::Value,
}

/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct SendmanyParams {
    pub dummy: Option<String>,
    pub amounts: serde_json::Value,
    pub minconf: u32,
    pub comment: String,
    pub subtractfeefrom: Vec<serde_json::Value>,
    pub replaceable: bool,
    pub conf_target: u64,
    pub estimate_mode: String,
    pub fee_rate: f64,
    pub verbose: bool,
}

/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.
#[derive(Debug, Serialize)]
pub struct SendmsgtopeerParams {
    pub peer_id: u64,
    pub msg_type: String,
    pub msg: String,
}

/// Submit a raw transaction (serialized, hex-encoded) to local node and network.
///
/// The transaction will be sent unconditionally to all peers, so using sendrawtransaction
/// for manual rebroadcast may degrade privacy by leaking the transaction"s origin, as
/// nodes will normally not rebroadcast non-wallet transactions already in their mempool.
///
/// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
///
/// Related RPCs: createrawtransaction, signrawtransactionwithkey
#[derive(Debug, Serialize)]
pub struct SendrawtransactionParams {
    pub hexstring: String,
    pub maxfeerate: f64,
    pub maxburnamount: f64,
}

/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct SendtoaddressParams {
    pub address: String,
    pub amount: bitcoin::Amount,
    pub comment: String,
    pub comment_to: String,
    pub subtractfeefromamount: bool,
    pub replaceable: bool,
    pub conf_target: u64,
    pub estimate_mode: String,
    pub avoid_reuse: bool,
    pub fee_rate: f64,
    pub verbose: bool,
}

/// Attempts to add or remove an IP/Subnet from the banned list.
#[derive(Debug, Serialize)]
pub struct SetbanParams {
    pub subnet: String,
    pub command: String,
    pub bantime: u64,
    pub absolute: bool,
}

/// Sets the label associated with the given address.
#[derive(Debug, Serialize)]
pub struct SetlabelParams {
    pub address: String,
    pub label: String,
}

/// Set the local time to given timestamp (-regtest only)
#[derive(Debug, Serialize)]
pub struct SetmocktimeParams {
    pub timestamp: u64,
}

/// Disable/enable all p2p network activity.
#[derive(Debug, Serialize)]
pub struct SetnetworkactiveParams {
    pub state: bool,
}

/// (DEPRECATED) Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
/// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
#[derive(Debug, Serialize)]
pub struct SettxfeeParams {
    pub amount: bitcoin::Amount,
}

/// Change the state of the given wallet flag for a wallet.
#[derive(Debug, Serialize)]
pub struct SetwalletflagParams {
    pub flag: String,
    pub value: bool,
}

/// Sign a message with the private key of an address
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct SignmessageParams {
    pub address: String,
    pub message: String,
}

/// Sign a message with the private key of an address
#[derive(Debug, Serialize)]
pub struct SignmessagewithprivkeyParams {
    pub privkey: String,
    pub message: String,
}

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
#[derive(Debug, Serialize)]
pub struct SignrawtransactionwithkeyParams {
    pub hexstring: String,
    pub privkeys: Vec<String>,
    pub prevtxs: Vec<serde_json::Value>,
    pub sighashtype: String,
}

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct SignrawtransactionwithwalletParams {
    pub hexstring: String,
    pub prevtxs: Vec<serde_json::Value>,
    pub sighashtype: String,
}

/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
#[derive(Debug, Serialize)]
pub struct SimulaterawtransactionParams {
    pub rawtxs: Vec<serde_json::Value>,
    pub options: serde_json::Value,
}

/// Request a graceful shutdown of Bitcoin Core.
#[derive(Debug, Serialize)]
pub struct StopParams {
    pub wait: u64,
}

/// Attempts to submit new block to network.
/// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
#[derive(Debug, Serialize)]
pub struct SubmitblockParams {
    pub hexdata: String,
    pub dummy: Option<String>,
}

/// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
/// Throws when the header is invalid.
#[derive(Debug, Serialize)]
pub struct SubmitheaderParams {
    pub hexdata: String,
}

/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
#[derive(Debug, Serialize)]
pub struct SubmitpackageParams {
    pub package: Vec<serde_json::Value>,
    pub maxfeerate: f64,
    pub maxburnamount: f64,
}

/// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
///
/// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
///
/// If one transaction fails, other transactions may not be fully validated (the "allowed" key will be blank).
///
/// The maximum number of transactions allowed is 25.
///
/// This checks if transactions violate the consensus or policy rules.
///
/// See sendrawtransaction call.
#[derive(Debug, Serialize)]
pub struct TestmempoolacceptParams {
    pub rawtxs: Vec<serde_json::Value>,
    pub maxfeerate: f64,
}

/// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
/// If both are specified, they must be identical.
#[derive(Debug, Serialize)]
pub struct UnloadwalletParams {
    pub wallet_name: String,
    pub load_on_startup: bool,
}

/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
#[derive(Debug, Serialize)]
pub struct UtxoupdatepsbtParams {
    pub psbt: String,
    pub descriptors: Vec<serde_json::Value>,
}

/// Return information about the given bitcoin address.
#[derive(Debug, Serialize)]
pub struct ValidateaddressParams {
    pub address: String,
}

/// Verifies blockchain database.
#[derive(Debug, Serialize)]
pub struct VerifychainParams {
    pub checklevel: u32,
    pub nblocks: u64,
}

/// Verify a signed message.
#[derive(Debug, Serialize)]
pub struct VerifymessageParams {
    pub address: String,
    pub signature: String,
    pub message: String,
}

/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
#[derive(Debug, Serialize)]
pub struct VerifytxoutproofParams {
    pub proof: String,
}

/// Waits for a specific new block and returns useful info about it.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Serialize)]
pub struct WaitforblockParams {
    pub blockhash: bitcoin::BlockHash,
    pub timeout: u64,
}

/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Serialize)]
pub struct WaitforblockheightParams {
    pub height: u64,
    pub timeout: u64,
}

/// Waits for any new block and returns useful info about it.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Serialize)]
pub struct WaitfornewblockParams {
    pub timeout: u64,
    pub current_tip: String,
}

/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
#[derive(Debug, Serialize)]
pub struct WalletcreatefundedpsbtParams {
    pub inputs: Vec<serde_json::Value>,
    pub outputs: Vec<serde_json::Value>,
    pub locktime: u32,
    pub options: serde_json::Value,
    pub bip32derivs: bool,
    pub version: u32,
}

/// Display address on an external signer for verification.
#[derive(Debug, Serialize)]
pub struct WalletdisplayaddressParams {
    pub address: String,
}

/// Stores the wallet decryption key in memory for "timeout" seconds.
/// This is needed prior to performing transactions related to private keys such as sending bitcoins
///
/// Note:
/// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
/// time that overrides the old one.
#[derive(Debug, Serialize)]
pub struct WalletpassphraseParams {
    pub passphrase: String,
    pub timeout: u64,
}

/// Changes the wallet passphrase from "oldpassphrase" to "newpassphrase".
#[derive(Debug, Serialize)]
pub struct WalletpassphrasechangeParams {
    pub oldpassphrase: String,
    pub newpassphrase: String,
}

/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct WalletprocesspsbtParams {
    pub psbt: String,
    pub sign: bool,
    pub sighashtype: String,
    pub bip32derivs: bool,
    pub finalize: bool,
}
