use serde::{Deserialize, Serialize};

/// Response for the `abortrescan` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AbortrescanResponse {
    /// Whether the abort was successful
    pub field_0: bool,
}

/// Response for the `addconnection` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddconnectionResponse {
    /// Address of newly added connection.
    pub address: String,
    /// Type of connection opened.
    pub connection_type: String,
}

/// Response for the `addmultisigaddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddmultisigaddressResponse {
    /// The value of the new multisig address
    pub address: String,
    /// The string value of the hex-encoded redemption script
    pub redeem_script: String,
    /// The descriptor for this multisig
    pub descriptor: String,
    /// Any warnings resulting from the creation of this multisig
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// Response for the `addpeeraddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddpeeraddressResponse {
    /// whether the peer address was successfully added to the address manager table
    pub success: bool,
    /// error description, if the address could not be added
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Response for the `analyzepsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AnalyzepsbtResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<serde_json::Value>>,
    /// Estimated vsize of the final signed transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_vsize: Option<u64>,
    /// Estimated feerate of the final signed transaction in BTC/kvB. Shown only if all UTXO slots in the PSBT have been filled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_feerate: Option<bitcoin::Amount>,
    /// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<bitcoin::Amount>,
    /// Role of the next person that this psbt needs to go to
    pub next: String,
    /// Error message (if there is one)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Response for the `bumpfee` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BumpfeeResponse {
    /// The id of the new transaction.
    pub txid: bitcoin::Txid,
    /// The fee of the replaced transaction.
    pub origfee: bitcoin::Amount,
    /// The fee of the new transaction.
    pub fee: bitcoin::Amount,
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
}

/// Response for the `combinepsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CombinepsbtResponse {
    /// The base64-encoded partially signed transaction
    pub field_0: String,
}

/// Response for the `combinerawtransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CombinerawtransactionResponse {
    /// The hex-encoded raw transaction with signature(s)
    pub field_0: String,
}

/// Response for the `converttopsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ConverttopsbtResponse {
    /// The resulting raw transaction (base64-encoded string)
    pub field_0: String,
}

/// Response for the `createmultisig` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatemultisigResponse {
    /// The value of the new multisig address.
    pub address: String,
    /// The string value of the hex-encoded redemption script.
    pub redeem_script: String,
    /// The descriptor for this multisig
    pub descriptor: String,
    /// Any warnings resulting from the creation of this multisig
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// Response for the `createpsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatepsbtResponse {
    /// The resulting raw transaction (base64-encoded string)
    pub field_0: String,
}

/// Response for the `createrawtransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreaterawtransactionResponse {
    /// hex string of the transaction
    pub transaction: String,
}

/// Response for the `createwallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatewalletResponse {
    /// The wallet name if created successfully. If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// Response for the `createwalletdescriptor` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatewalletdescriptorResponse {
    /// The public descriptors that were added to the wallet
    pub descs: Vec<serde_json::Value>,
}

/// Response for the `decodepsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecodepsbtResponse {
    /// The decoded network-serialized unsigned transaction.
    pub tx: serde_json::Value,
    pub global_xpubs: Vec<serde_json::Value>,
    /// The PSBT version number. Not to be confused with the unsigned transaction version
    pub psbt_version: u64,
    /// The global proprietary map
    pub proprietary: Vec<serde_json::Value>,
    /// The unknown global fields
    pub unknown: serde_json::Value,
    pub inputs: Vec<serde_json::Value>,
    pub outputs: Vec<serde_json::Value>,
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<bitcoin::Amount>,
}

/// Response for the `decoderawtransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecoderawtransactionResponse {
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The serialized transaction size
    pub size: u64,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: u64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: u64,
    /// The version
    pub version: u64,
    /// The lock time
    pub locktime: serde_json::Value,
    pub vin: Vec<serde_json::Value>,
    pub vout: Vec<serde_json::Value>,
}

/// Response for the `decodescript` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecodescriptResponse {
    /// Disassembly of the script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// The output type (e.g. nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    pub r#type: String,
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// address of P2SH script wrapping this redeem script (not returned for types that should not be wrapped)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2sh: Option<String>,
    /// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segwit: Option<serde_json::Value>,
}

/// Response for the `deriveaddresses` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DeriveaddressesResponse {
    pub field_0: Vec<serde_json::Value>,
    /// The derived addresses for each of the multipath expansions of the descriptor, in multipath specifier order
    pub field_1: Vec<serde_json::Value>,
}

/// Response for the `descriptorprocesspsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DescriptorprocesspsbtResponse {
    /// The base64-encoded partially signed transaction
    pub psbt: String,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

/// Response for the `dumpprivkey` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DumpprivkeyResponse {
    /// The private key
    pub key: String,
}

/// Response for the `dumptxoutset` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DumptxoutsetResponse {
    /// the number of coins written in the snapshot
    pub coins_written: u64,
    /// the hash of the base of the snapshot
    pub base_hash: String,
    /// the height of the base of the snapshot
    pub base_height: u64,
    /// the absolute path that the snapshot was written to
    pub path: String,
    /// the hash of the UTXO set contents
    pub txoutset_hash: String,
    /// the number of transactions in the chain up to and including the base block
    pub nchaintx: u64,
}

/// Response for the `dumpwallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DumpwalletResponse {
    /// The filename with full absolute path
    pub filename: String,
}

/// Response for the `echo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EchoResponse {
    /// Returns whatever was passed in
    pub field_0: serde_json::Value,
}

/// Response for the `echoipc` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EchoipcResponse {
    /// The echoed string.
    pub echo: String,
}

/// Response for the `echojson` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EchojsonResponse {
    /// Returns whatever was passed in
    pub field_0: serde_json::Value,
}

/// Response for the `encryptwallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EncryptwalletResponse {
    /// A string with further instructions
    pub field_0: String,
}

/// Response for the `enumeratesigners` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EnumeratesignersResponse {
    pub signers: Vec<serde_json::Value>,
}

/// Response for the `estimaterawfee` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EstimaterawfeeResponse {
    /// estimate for short time horizon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short: Option<serde_json::Value>,
    /// estimate for medium time horizon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<serde_json::Value>,
    /// estimate for long time horizon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long: Option<serde_json::Value>,
}

/// Response for the `estimatesmartfee` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EstimatesmartfeeResponse {
    /// estimate fee rate in BTC/kvB (only present if no errors were encountered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feerate: Option<u64>,
    /// Errors encountered during processing (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// block number where estimate was found
    /// The request target will be clamped between 2 and the highest target
    /// fee estimation is able to return based on how long it has been running.
    /// An error is returned if not enough transactions and blocks
    /// have been observed to make an estimate for any number of blocks.
    pub blocks: u64,
}

/// Response for the `finalizepsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FinalizepsbtResponse {
    /// The base64-encoded partially signed transaction if not extracted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
    /// The hex-encoded network transaction if extracted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
}

/// Response for the `fundrawtransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FundrawtransactionResponse {
    /// The resulting raw transaction (hex-encoded string)
    pub hex: String,
    /// Fee in BTC the resulting transaction pays
    pub fee: bitcoin::Amount,
    /// The position of the added change output, or -1
    pub changepos: u64,
}

/// Response for the `generateblock` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GenerateblockResponse {
    /// hash of generated block
    pub hash: String,
    /// hex of generated block, only present when submit=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

/// Response for the `generatetoaddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GeneratetoaddressResponse {
    /// hashes of blocks generated
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `generatetodescriptor` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GeneratetodescriptorResponse {
    /// hashes of blocks generated
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `getaddednodeinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetaddednodeinfoResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `getaddressesbylabel` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetaddressesbylabelResponse {
    /// json object with addresses as keys
    pub field_0: serde_json::Value,
}

/// Response for the `getaddressinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetaddressinfoResponse {
    /// The bitcoin address validated.
    pub address: String,
    /// The hex-encoded output script generated by the address.
    pub script_pub_key: bitcoin::ScriptBuf,
    /// If the address is yours.
    pub ismine: bool,
    /// If the address is watchonly.
    pub iswatchonly: bool,
    /// If we know how to spend coins sent to this address, ignoring the possible lack of private keys.
    pub solvable: bool,
    /// A descriptor for spending coins sent to this address (only when solvable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// The descriptor used to derive this address if this is a descriptor wallet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_desc: Option<String>,
    /// If the key is a script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isscript: Option<bool>,
    /// If the address was used for change output.
    pub ischange: bool,
    /// If the address is a witness address.
    pub iswitness: bool,
    /// The version number of the witness program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<u64>,
    /// The hex value of the witness program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
    /// The output script type. Only if isscript is true and the redeemscript is known. Possible
    /// types: nonstandard, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_keyhash,
    /// witness_v0_scripthash, witness_unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    /// The redeemscript for the p2sh address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// Array of pubkeys associated with the known redeemscript (only if script is multisig).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkeys: Option<Vec<serde_json::Value>>,
    /// The number of signatures required to spend multisig output (only if script is multisig).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sigsrequired: Option<u64>,
    /// The hex value of the raw public key for single-key addresses (possibly embedded in P2SH or P2WSH).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkey: Option<bitcoin::PublicKey>,
    /// Information about the address embedded in P2SH or P2WSH, if relevant and known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded: Option<serde_json::Value>,
    /// If the pubkey is compressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscompressed: Option<bool>,
    /// The creation time of the key, if available, expressed in UNIX epoch time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<serde_json::Value>,
    /// The HD keypath, if the key is HD and available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdkeypath: Option<String>,
    /// The Hash160 of the HD seed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdseedid: Option<String>,
    /// The fingerprint of the master key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdmasterfingerprint: Option<String>,
    /// Array of labels associated with the address. Currently limited to one label but returned
    /// as an array to keep the API stable if multiple labels are enabled in the future.
    pub labels: Vec<serde_json::Value>,
}

/// Response for the `getaddrmaninfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetaddrmaninfoResponse {
    /// json object with network type as keys
    pub field_0: serde_json::Value,
}

/// Response for the `getbalance` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbalanceResponse {
    /// The total amount in BTC received for this wallet.
    pub amount: bitcoin::Amount,
}

/// Response for the `getbalances` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbalancesResponse {
    /// balances from outputs that the wallet can sign
    pub mine: serde_json::Value,
    /// watchonly balances (not present if wallet does not watch anything)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchonly: Option<serde_json::Value>,
    /// hash and height of the block this information was generated on
    pub lastprocessedblock: bitcoin::Block,
}

/// Response for the `getbestblockhash` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbestblockhashResponse {
    /// the block hash, hex-encoded
    pub field_0: String,
}

/// Response for the `getblock` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockResponse {
    /// A string that is serialized, hex-encoded data for block 'hash'
    pub field_0: String,
    pub field_1: serde_json::Value,
    pub field_2: serde_json::Value,
    pub field_3: serde_json::Value,
}

/// Response for the `getblockchaininfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockchaininfoResponse {
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// the height of the most-work fully-validated chain. The genesis block has height 0
    pub blocks: u64,
    /// the current number of headers we have validated
    pub headers: u64,
    /// the hash of the currently best block
    pub bestblockhash: String,
    /// the current difficulty
    pub difficulty: u64,
    /// The block time expressed in UNIX epoch time
    pub time: serde_json::Value,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: serde_json::Value,
    /// estimate of verification progress [0..1]
    pub verificationprogress: u64,
    /// (debug information) estimate of whether this node is in Initial Block Download mode
    pub initialblockdownload: bool,
    /// total amount of work in active chain, in hexadecimal
    pub chainwork: String,
    /// the estimated size of the block and undo files on disk
    pub size_on_disk: u64,
    /// if the blocks are subject to pruning
    pub pruned: bool,
    /// height of the last block pruned, plus one (only present if pruning is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pruneheight: Option<u64>,
    /// whether automatic pruning is enabled (only present if pruning is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_pruning: Option<bool>,
    /// the target size used by pruning (only present if automatic pruning is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune_target_size: Option<u64>,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// Response for the `getblockcount` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockcountResponse {
    /// The current block count
    pub field_0: u64,
}

/// Response for the `getblockfilter` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockfilterResponse {
    /// the hex-encoded filter data
    pub filter: String,
    /// the hex-encoded filter header
    pub header: String,
}

/// Response for the `getblockhash` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockhashResponse {
    /// The block hash
    pub field_0: String,
}

/// Response for the `getblockheader` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockheaderResponse {
    pub field_0: serde_json::Value,
    /// A string that is serialized, hex-encoded data for block 'hash'
    pub field_1: String,
}

/// Response for the `getblockstats` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockstatsResponse {
    /// Average fee in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgfee: Option<u64>,
    /// Average feerate (in satoshis per virtual byte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgfeerate: Option<u64>,
    /// Average transaction size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgtxsize: Option<u64>,
    /// The block hash (to check for potential reorgs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<bitcoin::BlockHash>,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feerate_percentiles: Option<serde_json::Value>,
    /// The height of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    /// The number of inputs (excluding coinbase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins: Option<u64>,
    /// Maximum fee in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfee: Option<u64>,
    /// Maximum feerate (in satoshis per virtual byte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfeerate: Option<u64>,
    /// Maximum transaction size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxtxsize: Option<u64>,
    /// Truncated median fee in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medianfee: Option<u64>,
    /// The block median time past
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediantime: Option<u64>,
    /// Truncated median transaction size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediantxsize: Option<u64>,
    /// Minimum fee in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minfee: Option<u64>,
    /// Minimum feerate (in satoshis per virtual byte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minfeerate: Option<u64>,
    /// Minimum transaction size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mintxsize: Option<u64>,
    /// The number of outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outs: Option<u64>,
    /// The block subsidy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsidy: Option<u64>,
    /// Total size of all segwit transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtotal_size: Option<u64>,
    /// Total weight of all segwit transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtotal_weight: Option<u64>,
    /// The number of segwit transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtxs: Option<u64>,
    /// The block time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    /// Total amount in all outputs (excluding coinbase and thus reward [ie subsidy + totalfee])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_out: Option<u64>,
    /// Total size of all non-coinbase transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
    /// Total weight of all non-coinbase transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_weight: Option<u64>,
    /// The fee total
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalfee: Option<u64>,
    /// The number of transactions (including coinbase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs: Option<u64>,
    /// The increase/decrease in the number of unspent outputs (not discounting op_return and similar)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase: Option<u64>,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc: Option<u64>,
    /// The increase/decrease in the number of unspent outputs, not counting unspendables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase_actual: Option<u64>,
    /// The increase/decrease in size for the utxo index, not counting unspendables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc_actual: Option<u64>,
}

/// Response for the `getblocktemplate` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblocktemplateResponse {
    /// According to BIP22
    pub field_1: String,
    pub field_2: serde_json::Value,
}

/// Response for the `getchainstates` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetchainstatesResponse {
    /// the number of headers seen so far
    pub headers: u64,
    /// list of the chainstates ordered by work, with the most-work (active) chainstate last
    pub chainstates: Vec<serde_json::Value>,
}

/// Response for the `getchaintips` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetchaintipsResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `getchaintxstats` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetchaintxstatsResponse {
    /// The timestamp for the final block in the window, expressed in UNIX epoch time
    pub time: serde_json::Value,
    /// The total number of transactions in the chain up to that point, if known. It may be unknown when using assumeutxo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txcount: Option<u64>,
    /// The hash of the final block in the window
    pub window_final_block_hash: String,
    /// The height of the final block in the window.
    pub window_final_block_height: u64,
    /// Size of the window in number of blocks
    pub window_block_count: u64,
    /// The elapsed time in the window in seconds. Only returned if \"window_block_count\" is > 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_interval: Option<u64>,
    /// The number of transactions in the window. Only returned if \"window_block_count\" is > 0 and if txcount exists for the start and end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_tx_count: Option<u64>,
    /// The average rate of transactions per second in the window. Only returned if \"window_interval\" is > 0 and if window_tx_count exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txrate: Option<u64>,
}

/// Response for the `getconnectioncount` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetconnectioncountResponse {
    /// The connection count
    pub field_0: u64,
}

/// Response for the `getdeploymentinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetdeploymentinfoResponse {
    /// requested block hash (or tip)
    pub hash: String,
    /// requested block height (or tip)
    pub height: u64,
    pub deployments: serde_json::Value,
}

/// Response for the `getdescriptorinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetdescriptorinfoResponse {
    /// The descriptor in canonical form, without private keys. For a multipath descriptor, only the first will be returned.
    pub descriptor: String,
    /// All descriptors produced by expanding multipath derivation elements. Only if the provided descriptor specifies multipath derivation elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipath_expansion: Option<Vec<serde_json::Value>>,
    /// The checksum for the input descriptor
    pub checksum: String,
    /// Whether the descriptor is ranged
    pub isrange: bool,
    /// Whether the descriptor is solvable
    pub issolvable: bool,
    /// Whether the input descriptor contained at least one private key
    pub hasprivatekeys: bool,
}

/// Response for the `getdifficulty` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetdifficultyResponse {
    /// the proof-of-work difficulty as a multiple of the minimum difficulty.
    pub field_0: u64,
}

/// Response for the `gethdkeys` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GethdkeysResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `getindexinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetindexinfoResponse {
    pub field_0: serde_json::Value,
}

/// Response for the `getmemoryinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmemoryinfoResponse {
    pub field_0: serde_json::Value,
    /// \"<malloc version=\"1\">...\"
    pub field_1: String,
}

/// Response for the `getmempoolancestors` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmempoolancestorsResponse {
    pub field_0: Vec<serde_json::Value>,
    pub field_1: serde_json::Value,
}

/// Response for the `getmempooldescendants` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmempooldescendantsResponse {
    pub field_0: Vec<serde_json::Value>,
    pub field_1: serde_json::Value,
}

/// Response for the `getmempoolentry` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmempoolentryResponse {
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: u64,
    /// transaction weight as defined in BIP 141.
    pub weight: u64,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: serde_json::Value,
    /// block height when transaction entered pool
    pub height: u64,
    /// number of in-mempool descendant transactions (including this one)
    pub descendantcount: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    pub descendantsize: u64,
    /// number of in-mempool ancestor transactions (including this one)
    pub ancestorcount: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    pub ancestorsize: u64,
    /// hash of serialized transaction, including witness data
    pub wtxid: bitcoin::Txid,
    pub fees: serde_json::Value,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<serde_json::Value>,
    /// unconfirmed transactions spending outputs from this transaction
    pub spentby: Vec<serde_json::Value>,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
    pub bip125_replaceable: bool,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
}

/// Response for the `getmempoolinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmempoolinfoResponse {
    /// True if the initial load attempt of the persisted mempool finished
    pub loaded: bool,
    /// Current tx count
    pub size: u64,
    pub bytes: u64,
    /// Total memory usage for the mempool
    pub usage: u64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction
    pub total_fee: bitcoin::Amount,
    /// Maximum memory usage for the mempool
    pub maxmempool: u64,
    /// Minimum fee rate in BTC/kvB for tx to be accepted. Is the maximum of minrelaytxfee and minimum mempool fee
    pub mempoolminfee: bitcoin::Amount,
    /// Current minimum relay fee for transactions
    pub minrelaytxfee: bitcoin::Amount,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    pub incrementalrelayfee: u64,
    /// Current number of transactions that haven't passed initial broadcast yet
    pub unbroadcastcount: u64,
    /// True if the mempool accepts RBF without replaceability signaling inspection (DEPRECATED)
    pub fullrbf: bool,
}

/// Response for the `getmininginfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmininginfoResponse {
    /// The current block
    pub blocks: u64,
    /// The block weight of the last assembled block (only present if a block was ever assembled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentblockweight: Option<u64>,
    /// The number of block transactions of the last assembled block (only present if a block was ever assembled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentblocktx: Option<u64>,
    /// The current difficulty
    pub difficulty: u64,
    /// The network hashes per second
    pub networkhashps: u64,
    /// The size of the mempool
    pub pooledtx: u64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// Response for the `getnettotals` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnettotalsResponse {
    /// Total bytes received
    pub totalbytesrecv: u64,
    /// Total bytes sent
    pub totalbytessent: u64,
    /// Current system UNIX epoch time in milliseconds
    pub timemillis: serde_json::Value,
    pub uploadtarget: serde_json::Value,
}

/// Response for the `getnetworkhashps` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnetworkhashpsResponse {
    /// Hashes per second estimated
    pub field_0: u64,
}

/// Response for the `getnetworkinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnetworkinfoResponse {
    /// the server version
    pub version: u64,
    /// the server subversion string
    pub subversion: String,
    /// the protocol version
    pub protocolversion: u64,
    /// the services we offer to the network
    pub localservices: String,
    /// the services we offer to the network, in human-readable form
    pub localservicesnames: Vec<serde_json::Value>,
    /// true if transaction relay is requested from peers
    pub localrelay: bool,
    /// the time offset
    pub timeoffset: u64,
    /// the total number of connections
    pub connections: u64,
    /// the number of inbound connections
    pub connections_in: u64,
    /// the number of outbound connections
    pub connections_out: u64,
    /// whether p2p networking is enabled
    pub networkactive: bool,
    /// information per network
    pub networks: Vec<serde_json::Value>,
    /// minimum relay fee rate for transactions in BTC/kvB
    pub relayfee: u64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    pub incrementalfee: u64,
    /// list of local addresses
    pub localaddresses: Vec<bitcoin::Address<bitcoin::address::NetworkUnchecked>>,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// Response for the `getnewaddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnewaddressResponse {
    /// The new bitcoin address
    pub address: String,
}

/// Response for the `getnodeaddresses` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnodeaddressesResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `getorphantxs` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetorphantxsResponse {
    pub field_0: Vec<serde_json::Value>,
    pub field_1: Vec<serde_json::Value>,
    pub field_2: Vec<serde_json::Value>,
}

/// Response for the `getpeerinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetpeerinfoResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `getprioritisedtransactions` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetprioritisedtransactionsResponse {
    /// prioritisation keyed by txid
    pub field_0: serde_json::Value,
}

/// Response for the `getrawaddrman` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrawaddrmanResponse {
    pub field_0: serde_json::Value,
}

/// Response for the `getrawchangeaddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrawchangeaddressResponse {
    /// The address
    pub address: String,
}

/// Response for the `getrawmempool` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrawmempoolResponse {
    pub field_0: Vec<serde_json::Value>,
    pub field_1: serde_json::Value,
    pub field_2: serde_json::Value,
}

/// Response for the `getrawtransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrawtransactionResponse {
    /// The serialized transaction as a hex-encoded string for 'txid'
    pub data: String,
    pub field_1: serde_json::Value,
    pub field_2: serde_json::Value,
}

/// Response for the `getreceivedbyaddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetreceivedbyaddressResponse {
    /// The total amount in BTC received at this address.
    pub amount: bitcoin::Amount,
}

/// Response for the `getreceivedbylabel` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetreceivedbylabelResponse {
    /// The total amount in BTC received for this label.
    pub amount: bitcoin::Amount,
}

/// Response for the `getrpcinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrpcinfoResponse {
    /// All active commands
    pub active_commands: Vec<serde_json::Value>,
    /// The complete file path to the debug log
    pub logpath: String,
}

/// Response for the `gettransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettransactionResponse {
    /// The amount in BTC
    pub amount: bitcoin::Amount,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<bitcoin::Amount>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: u64,
    /// Only present if the transaction's only input is a coinbase one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<bool>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
    /// The block hash containing the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<bitcoin::BlockHash>,
    /// The block height containing the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockheight: Option<u64>,
    /// The index of the transaction in the block that includes it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockindex: Option<u64>,
    /// The block time expressed in UNIX epoch time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocktime: Option<serde_json::Value>,
    /// The transaction id.
    pub txid: bitcoin::Txid,
    /// The hash of serialized transaction, including witness data.
    pub wtxid: bitcoin::Txid,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    pub walletconflicts: Vec<serde_json::Value>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by_txid: Option<bitcoin::Txid>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_txid: Option<bitcoin::Txid>,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    pub mempoolconflicts: Vec<serde_json::Value>,
    /// If a comment to is associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: serde_json::Value,
    /// The time received expressed in UNIX epoch time.
    pub timereceived: serde_json::Value,
    /// If a comment is associated with the transaction, only present if not empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// (\"yes|no|unknown\") Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    pub bip125_replaceable: String,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_descs: Option<Vec<serde_json::Value>>,
    pub details: Vec<serde_json::Value>,
    /// Raw data for transaction
    pub hex: String,
    /// The decoded transaction (only present when `verbose` is passed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded: Option<serde_json::Value>,
    /// hash and height of the block this information was generated on
    pub lastprocessedblock: bitcoin::Block,
}

/// Response for the `gettxout` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettxoutResponse {
    pub field_1: serde_json::Value,
}

/// Response for the `gettxoutproof` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettxoutproofResponse {
    /// A string that is a serialized, hex-encoded data for the proof.
    pub data: String,
}

/// Response for the `gettxoutsetinfo` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettxoutsetinfoResponse {
    /// The block height (index) of the returned statistics
    pub height: u64,
    /// The hash of the block at which these statistics are calculated
    pub bestblock: String,
    /// The number of unspent transaction outputs
    pub txouts: u64,
    /// Database-independent, meaningless metric indicating the UTXO set size
    pub bogosize: u64,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_serialized_3: Option<String>,
    /// The serialized hash (only present if 'muhash' hash_type is chosen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muhash: Option<String>,
    /// The number of transactions with unspent outputs (not available when coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<u64>,
    /// The estimated size of the chainstate on disk (not available when coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<u64>,
    /// The total amount of coins in the UTXO set
    pub total_amount: bitcoin::Amount,
    /// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_unspendable_amount: Option<bitcoin::Amount>,
    /// Info on amounts in the block at this block height (only available if coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_info: Option<bitcoin::Block>,
}

/// Response for the `gettxspendingprevout` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettxspendingprevoutResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `getunconfirmedbalance` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetunconfirmedbalanceResponse {
    /// The balance
    pub field_0: u64,
}

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
    pub external_signer: bool,
    /// Whether this wallet intentionally does not contain any keys, scripts, or descriptors
    pub blank: bool,
    /// The start time for blocks scanning. It could be modified by (re)importing any descriptor with an earlier timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthtime: Option<serde_json::Value>,
    /// hash and height of the block this information was generated on
    pub lastprocessedblock: bitcoin::Block,
}

/// Response for the `help` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct HelpResponse {
    /// The help text
    pub field_0: String,
    pub field_1: serde_json::Value,
}

/// Response for the `importdescriptors` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ImportdescriptorsResponse {
    /// Response is an array with the same size as the input that has the execution result
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `importmulti` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ImportmultiResponse {
    /// Response is an array with the same size as the input that has the execution result
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `joinpsbts` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct JoinpsbtsResponse {
    /// The base64-encoded partially signed transaction
    pub field_0: String,
}

/// Response for the `listaddressgroupings` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListaddressgroupingsResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `listbanned` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListbannedResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `listdescriptors` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListdescriptorsResponse {
    /// Name of wallet this operation was performed on
    pub wallet_name: String,
    /// Array of descriptor objects (sorted by descriptor string representation)
    pub descriptors: Vec<bitcoin::ScriptBuf>,
}

/// Response for the `listlabels` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListlabelsResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `listlockunspent` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListlockunspentResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `listreceivedbyaddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListreceivedbyaddressResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `listreceivedbylabel` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListreceivedbylabelResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `listsinceblock` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListsinceblockResponse {
    pub transactions: Vec<serde_json::Value>,
    /// <structure is the same as \"transactions\" above, only present if include_removed=true>
    /// Note: transactions that were re-added in the active chain will appear as-is in this array, and may thus have a positive confirmation count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<serde_json::Value>>,
    pub lastblock: String,
}

/// Response for the `listtransactions` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListtransactionsResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `listunspent` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListunspentResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `listwalletdir` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListwalletdirResponse {
    pub wallets: Vec<serde_json::Value>,
}

/// Response for the `listwallets` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListwalletsResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `loadtxoutset` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LoadtxoutsetResponse {
    /// the number of coins loaded from the snapshot
    pub coins_loaded: u64,
    /// the hash of the base of the snapshot
    pub tip_hash: String,
    /// the height of the base of the snapshot
    pub base_height: u64,
    /// the absolute path that the snapshot was loaded from
    pub path: String,
}

/// Response for the `loadwallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LoadwalletResponse {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// Response for the `lockunspent` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LockunspentResponse {
    /// Whether the command was successful or not
    pub field_0: bool,
}

/// Response for the `logging` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LoggingResponse {
    /// keys are the logging categories, and values indicates its status
    pub field_0: serde_json::Value,
}

/// Response for the `migratewallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MigratewalletResponse {
    /// The name of the primary migrated wallet
    pub wallet_name: String,
    /// The name of the migrated wallet containing the watchonly scripts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchonly_name: Option<String>,
    /// The name of the migrated wallet containing solvable but not watched scripts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solvables_name: Option<String>,
    /// The location of the backup of the original wallet
    pub backup_path: String,
}

/// Response for the `prioritisetransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PrioritisetransactionResponse {
    /// Returns true
    pub field_0: bool,
}

/// Response for the `pruneblockchain` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PruneblockchainResponse {
    /// Height of the last block pruned
    pub field_0: u64,
}

/// Response for the `psbtbumpfee` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PsbtbumpfeeResponse {
    /// The base64-encoded unsigned PSBT of the new transaction.
    pub psbt: String,
    /// The fee of the replaced transaction.
    pub origfee: bitcoin::Amount,
    /// The fee of the new transaction.
    pub fee: bitcoin::Amount,
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
}

/// Response for the `rescanblockchain` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RescanblockchainResponse {
    /// The block height where the rescan started (the requested height or 0)
    pub start_height: u64,
    pub stop_height: u64,
}

/// Response for the `restorewallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RestorewalletResponse {
    /// The wallet name if restored successfully.
    pub name: String,
    /// Warning messages, if any, related to restoring and loading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// Response for the `savemempool` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SavemempoolResponse {
    /// the directory and file where the mempool was saved
    pub filename: String,
}

/// Response for the `scanblocks` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanblocksResponse {
    pub field_1: serde_json::Value,
    pub field_2: serde_json::Value,
    /// True if scan will be aborted (not necessarily before this RPC returns), or false if there is no scan to abort
    pub success: bool,
}

/// Response for the `scantxoutset` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScantxoutsetResponse {
    pub field_0: serde_json::Value,
    /// True if scan will be aborted (not necessarily before this RPC returns), or false if there is no scan to abort
    pub success: bool,
    pub field_2: serde_json::Value,
}

/// Response for the `send` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
}

/// Response for the `sendall` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendallResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
}

/// Response for the `sendmany` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendmanyResponse {
    /// The transaction id for the send. Only 1 transaction is created regardless of
    /// the number of addresses.
    pub txid: bitcoin::Txid,
    pub field_1: serde_json::Value,
}

/// Response for the `sendrawtransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendrawtransactionResponse {
    /// The transaction hash in hex
    pub field_0: String,
}

/// Response for the `sendtoaddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendtoaddressResponse {
    /// The transaction id.
    pub txid: bitcoin::Txid,
    pub field_1: serde_json::Value,
}

/// Response for the `setnetworkactive` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SetnetworkactiveResponse {
    /// The value that was passed in
    pub field_0: bool,
}

/// Response for the `settxfee` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SettxfeeResponse {
    /// Returns true if successful
    pub field_0: bool,
}

/// Response for the `setwalletflag` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SetwalletflagResponse {
    /// The name of the flag that was modified
    pub flag_name: String,
    /// The new state of the flag
    pub flag_state: bool,
    /// Any warnings associated with the change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<String>,
}

/// Response for the `signmessage` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SignmessageResponse {
    /// The signature of the message encoded in base 64
    pub signature: String,
}

/// Response for the `signmessagewithprivkey` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SignmessagewithprivkeyResponse {
    /// The signature of the message encoded in base 64
    pub signature: String,
}

/// Response for the `signrawtransactionwithkey` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SignrawtransactionwithkeyResponse {
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

/// Response for the `signrawtransactionwithwallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SignrawtransactionwithwalletResponse {
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

/// Response for the `simulaterawtransaction` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SimulaterawtransactionResponse {
    /// The wallet balance change (negative means decrease).
    pub balance_change: bitcoin::Amount,
}

/// Response for the `stop` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StopResponse {
    /// A string with the content 'Bitcoin Core stopping'
    pub field_0: String,
}

/// Response for the `submitblock` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitblockResponse {
    /// According to BIP22
    pub field_1: String,
}

/// Response for the `submitpackage` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitpackageResponse {
    /// The transaction package result message. \"success\" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// transaction results keyed by wtxid
    pub tx_results: serde_json::Value,
    /// List of txids of replaced transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_transactions: Option<Vec<serde_json::Value>>,
}

/// Response for the `testmempoolaccept` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TestmempoolacceptResponse {
    /// The result of the mempool acceptance test for each raw transaction in the input array.
    /// Returns results for each transaction in the same order they were passed in.
    /// Transactions that cannot be fully validated due to failures in other transactions will not contain an 'allowed' result.
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `unloadwallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UnloadwalletResponse {
    /// Warning messages, if any, related to unloading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// Response for the `upgradewallet` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradewalletResponse {
    /// Name of wallet this operation was performed on
    pub wallet_name: String,
    /// Version of wallet before this operation
    pub previous_version: u64,
    /// Version of wallet after this operation
    pub current_version: u64,
    /// Description of result, if no error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// Error message (if there is one)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Response for the `uptime` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UptimeResponse {
    /// The number of seconds that the server has been running
    pub field_0: u64,
}

/// Response for the `utxoupdatepsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UtxoupdatepsbtResponse {
    /// The base64-encoded partially signed transaction with inputs updated
    pub field_0: String,
}

/// Response for the `validateaddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ValidateaddressResponse {
    /// If the address is valid or not
    pub isvalid: bool,
    /// The bitcoin address validated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The hex-encoded output script generated by the address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_pub_key: Option<bitcoin::ScriptBuf>,
    /// If the key is a script
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isscript: Option<bool>,
    /// If the address is a witness address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iswitness: Option<bool>,
    /// The version number of the witness program
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<u64>,
    /// The hex value of the witness program
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
    /// Error message, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Indices of likely error locations in address, if known (e.g. Bech32 errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_locations: Option<Vec<String>>,
}

/// Response for the `verifychain` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifychainResponse {
    /// Verification finished successfully. If false, check debug.log for reason.
    pub field_0: bool,
}

/// Response for the `verifymessage` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifymessageResponse {
    /// If the signature is verified or not.
    pub field_0: bool,
}

/// Response for the `verifytxoutproof` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifytxoutproofResponse {
    pub field_0: Vec<serde_json::Value>,
}

/// Response for the `waitforblock` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WaitforblockResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

/// Response for the `waitforblockheight` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WaitforblockheightResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

/// Response for the `waitfornewblock` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WaitfornewblockResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

/// Response for the `walletcreatefundedpsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletcreatefundedpsbtResponse {
    /// The resulting raw transaction (base64-encoded string)
    pub psbt: String,
    /// Fee in BTC the resulting transaction pays
    pub fee: bitcoin::Amount,
    /// The position of the added change output, or -1
    pub changepos: u64,
}

/// Response for the `walletdisplayaddress` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletdisplayaddressResponse {
    /// The address as confirmed by the signer
    pub address: String,
}

/// Response for the `walletprocesspsbt` RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletprocesspsbtResponse {
    /// The base64-encoded partially signed transaction
    pub psbt: String,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

