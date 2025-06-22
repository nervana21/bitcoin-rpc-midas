#[cfg(test)]
use anyhow::Result;
use std::sync::Arc;
use crate::transport::core::{TransportExt, TransportError};
use crate::transport::{DefaultTransport};
use crate::types::v28_types::*;
#[cfg(test)]
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct BitcoinNodeClient {
    client: Arc<DefaultTransport>,
}

impl BitcoinNodeClient {
    pub fn new(client: Arc<DefaultTransport>) -> Self {
        Self { client }
    }

    pub fn with_transport(&mut self, client: Arc<DefaultTransport>) {
        self.client = client;
    }

/// Open an outbound connection to a specified node. This RPC is for testing only.
    pub async fn addconnection(&self, address: String, connection_type: String, v2transport: bool) -> Result<AddconnectionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(connection_type)?);
        params.push(serde_json::to_value(v2transport)?);
        // dispatch and deserialize to `AddconnectionResponse`
        self.client.call::<AddconnectionResponse>("addconnection", &params).await
    }

/// Attempts to add or remove a node from the addnode list.
/// Or try a connection to a node once.
/// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
/// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
/// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
    pub async fn addnode(&self, node: String, command: String, v2transport: bool) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(node)?);
        params.push(serde_json::to_value(command)?);
        params.push(serde_json::to_value(v2transport)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("addnode", &params).await
    }

/// Add the address of a potential peer to an address manager table. This RPC is for testing only.
    pub async fn addpeeraddress(&self, address: String, port: u16, tried: bool) -> Result<AddpeeraddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(port)?);
        params.push(serde_json::to_value(tried)?);
        // dispatch and deserialize to `AddpeeraddressResponse`
        self.client.call::<AddpeeraddressResponse>("addpeeraddress", &params).await
    }

/// Analyzes and provides information about the current status of a PSBT and its inputs
    pub async fn analyzepsbt(&self, psbt: String) -> Result<AnalyzepsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(psbt)?);
        // dispatch and deserialize to `AnalyzepsbtResponse`
        self.client.call::<AnalyzepsbtResponse>("analyzepsbt", &params).await
    }

/// Return JSON description of RPC API.
    pub async fn api(&self) -> Result<ApiResponse, TransportError> {
        // dispatch and deserialize to `ApiResponse`
        self.client.call::<ApiResponse>("api", &[]).await
    }

/// Clear all banned IPs.
    pub async fn clearbanned(&self) -> Result<(), TransportError> {
        // dispatch and deserialize to `()`
        self.client.call::<()>("clearbanned", &[]).await
    }

/// Combine multiple partially signed Bitcoin transactions into one transaction.
/// Implements the Combiner role.
    pub async fn combinepsbt(&self, txs: Vec<serde_json::Value>) -> Result<CombinepsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txs)?);
        // dispatch and deserialize to `CombinepsbtResponse`
        self.client.call::<CombinepsbtResponse>("combinepsbt", &params).await
    }

/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.
    pub async fn combinerawtransaction(&self, txs: Vec<serde_json::Value>) -> Result<CombinerawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txs)?);
        // dispatch and deserialize to `CombinerawtransactionResponse`
        self.client.call::<CombinerawtransactionResponse>("combinerawtransaction", &params).await
    }

/// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// createpsbt and walletcreatefundedpsbt should be used for new applications.
    pub async fn converttopsbt(&self, hexstring: String, permitsigdata: bool, iswitness: bool) -> Result<ConverttopsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        params.push(serde_json::to_value(permitsigdata)?);
        params.push(serde_json::to_value(iswitness)?);
        // dispatch and deserialize to `ConverttopsbtResponse`
        self.client.call::<ConverttopsbtResponse>("converttopsbt", &params).await
    }

/// Creates a multi-signature address with n signature of m keys required.
/// It returns a json object with the address and redeemScript.
    pub async fn createmultisig(&self, nrequired: u32, keys: Vec<String>, address_type: String) -> Result<CreatemultisigResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(nrequired)?);
        params.push(serde_json::to_value(keys)?);
        params.push(serde_json::to_value(address_type)?);
        // dispatch and deserialize to `CreatemultisigResponse`
        self.client.call::<CreatemultisigResponse>("createmultisig", &params).await
    }

/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
    pub async fn createpsbt(&self, inputs: Vec<serde_json::Value>, outputs: Vec<serde_json::Value>, locktime: u32, replaceable: bool) -> Result<CreatepsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(inputs)?);
        params.push(serde_json::to_value(outputs)?);
        params.push(serde_json::to_value(locktime)?);
        params.push(serde_json::to_value(replaceable)?);
        // dispatch and deserialize to `CreatepsbtResponse`
        self.client.call::<CreatepsbtResponse>("createpsbt", &params).await
    }

/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction"s inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
    pub async fn createrawtransaction(&self, inputs: Vec<serde_json::Value>, outputs: Vec<serde_json::Value>, locktime: u32, replaceable: bool) -> Result<CreaterawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(inputs)?);
        params.push(serde_json::to_value(outputs)?);
        params.push(serde_json::to_value(locktime)?);
        params.push(serde_json::to_value(replaceable)?);
        // dispatch and deserialize to `CreaterawtransactionResponse`
        self.client.call::<CreaterawtransactionResponse>("createrawtransaction", &params).await
    }

/// Creates the wallet"s descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn createwalletdescriptor(&self, _type: String, options: serde_json::Value) -> Result<CreatewalletdescriptorResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(_type)?);
        params.push(serde_json::to_value(options)?);
        // dispatch and deserialize to `CreatewalletdescriptorResponse`
        self.client.call::<CreatewalletdescriptorResponse>("createwalletdescriptor", &params).await
    }

/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
    pub async fn decodepsbt(&self, psbt: String) -> Result<DecodepsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(psbt)?);
        // dispatch and deserialize to `DecodepsbtResponse`
        self.client.call::<DecodepsbtResponse>("decodepsbt", &params).await
    }

/// Return a JSON object representing the serialized, hex-encoded transaction.
    pub async fn decoderawtransaction(&self, hexstring: String, iswitness: bool) -> Result<DecoderawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        params.push(serde_json::to_value(iswitness)?);
        // dispatch and deserialize to `DecoderawtransactionResponse`
        self.client.call::<DecoderawtransactionResponse>("decoderawtransaction", &params).await
    }

/// Decode a hex-encoded script.
    pub async fn decodescript(&self, hexstring: String) -> Result<DecodescriptResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        // dispatch and deserialize to `DecodescriptResponse`
        self.client.call::<DecodescriptResponse>("decodescript", &params).await
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
    pub async fn deriveaddresses(&self, descriptor: String, range: serde_json::Value) -> Result<DeriveaddressesResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(descriptor)?);
        params.push(serde_json::to_value(range)?);
        // dispatch and deserialize to `DeriveaddressesResponse`
        self.client.call::<DeriveaddressesResponse>("deriveaddresses", &params).await
    }

/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
    pub async fn descriptorprocesspsbt(&self, psbt: String, descriptors: Vec<serde_json::Value>, sighashtype: String, bip32derivs: bool, finalize: bool) -> Result<DescriptorprocesspsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(psbt)?);
        params.push(serde_json::to_value(descriptors)?);
        params.push(serde_json::to_value(sighashtype)?);
        params.push(serde_json::to_value(bip32derivs)?);
        params.push(serde_json::to_value(finalize)?);
        // dispatch and deserialize to `DescriptorprocesspsbtResponse`
        self.client.call::<DescriptorprocesspsbtResponse>("descriptorprocesspsbt", &params).await
    }

/// Immediately disconnects from the specified peer node.
///
/// Strictly one out of "address" and "nodeid" can be provided to identify the node.
///
/// To disconnect by nodeid, either set "address" to the empty string, or call using the named "nodeid" argument only.
    pub async fn disconnectnode(&self, address: String, nodeid: u64) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(nodeid)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("disconnectnode", &params).await
    }

/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
///
/// Unless the the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
///
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn dumptxoutset(&self, path: String, _type: String, options: serde_json::Value) -> Result<DumptxoutsetResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(path)?);
        params.push(serde_json::to_value(_type)?);
        params.push(serde_json::to_value(options)?);
        // dispatch and deserialize to `DumptxoutsetResponse`
        self.client.call::<DumptxoutsetResponse>("dumptxoutset", &params).await
    }

/// Simply echo back the input arguments. This command is for testing.
///
/// It will return an internal bug report when arg9="trigger_internal_bug" is passed.
///
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    pub async fn echo(&self, arg0: String, arg1: String, arg2: String, arg3: String, arg4: String, arg5: String, arg6: String, arg7: String, arg8: String, arg9: String) -> Result<EchoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(arg0)?);
        params.push(serde_json::to_value(arg1)?);
        params.push(serde_json::to_value(arg2)?);
        params.push(serde_json::to_value(arg3)?);
        params.push(serde_json::to_value(arg4)?);
        params.push(serde_json::to_value(arg5)?);
        params.push(serde_json::to_value(arg6)?);
        params.push(serde_json::to_value(arg7)?);
        params.push(serde_json::to_value(arg8)?);
        params.push(serde_json::to_value(arg9)?);
        // dispatch and deserialize to `EchoResponse`
        self.client.call::<EchoResponse>("echo", &params).await
    }

/// Echo back the input argument, passing it through a spawned process in a multiprocess build.
/// This command is for testing.
    pub async fn echoipc(&self, arg: String) -> Result<EchoipcResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(arg)?);
        // dispatch and deserialize to `EchoipcResponse`
        self.client.call::<EchoipcResponse>("echoipc", &params).await
    }

/// Simply echo back the input arguments. This command is for testing.
///
/// It will return an internal bug report when arg9="trigger_internal_bug" is passed.
///
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    pub async fn echojson(&self, arg0: String, arg1: String, arg2: String, arg3: String, arg4: String, arg5: String, arg6: String, arg7: String, arg8: String, arg9: String) -> Result<EchojsonResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(arg0)?);
        params.push(serde_json::to_value(arg1)?);
        params.push(serde_json::to_value(arg2)?);
        params.push(serde_json::to_value(arg3)?);
        params.push(serde_json::to_value(arg4)?);
        params.push(serde_json::to_value(arg5)?);
        params.push(serde_json::to_value(arg6)?);
        params.push(serde_json::to_value(arg7)?);
        params.push(serde_json::to_value(arg8)?);
        params.push(serde_json::to_value(arg9)?);
        // dispatch and deserialize to `EchojsonResponse`
        self.client.call::<EchojsonResponse>("echojson", &params).await
    }

/// Returns a list of external signers from -signer.
    pub async fn enumeratesigners(&self) -> Result<EnumeratesignersResponse, TransportError> {
        // dispatch and deserialize to `EnumeratesignersResponse`
        self.client.call::<EnumeratesignersResponse>("enumeratesigners", &[]).await
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
    pub async fn estimaterawfee(&self, conf_target: u64, threshold: u64) -> Result<EstimaterawfeeResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(conf_target)?);
        params.push(serde_json::to_value(threshold)?);
        // dispatch and deserialize to `EstimaterawfeeResponse`
        self.client.call::<EstimaterawfeeResponse>("estimaterawfee", &params).await
    }

/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).
    pub async fn estimatesmartfee(&self, conf_target: u64, estimate_mode: String) -> Result<EstimatesmartfeeResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(conf_target)?);
        params.push(serde_json::to_value(estimate_mode)?);
        // dispatch and deserialize to `EstimatesmartfeeResponse`
        self.client.call::<EstimatesmartfeeResponse>("estimatesmartfee", &params).await
    }

/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptWitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
    pub async fn finalizepsbt(&self, psbt: String, extract: bool) -> Result<FinalizepsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(psbt)?);
        params.push(serde_json::to_value(extract)?);
        // dispatch and deserialize to `FinalizepsbtResponse`
        self.client.call::<FinalizepsbtResponse>("finalizepsbt", &params).await
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
/// in the wallet using importaddress or addmultisigaddress (to calculate fees).
/// You can see whether this is the case by checking the "solvable" field in the listunspent output.
/// Only pay-to-pubkey, multisig, and P2SH versions thereof are currently supported for watch-only
    pub async fn fundrawtransaction(&self, hexstring: String, options: serde_json::Value, iswitness: bool) -> Result<FundrawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        params.push(serde_json::to_value(options)?);
        params.push(serde_json::to_value(iswitness)?);
        // dispatch and deserialize to `FundrawtransactionResponse`
        self.client.call::<FundrawtransactionResponse>("fundrawtransaction", &params).await
    }

/// has been replaced by the -generate cli option. Refer to -help for more information.
    pub async fn generate(&self) -> Result<(), TransportError> {
        // dispatch and deserialize to `()`
        self.client.call::<()>("generate", &[]).await
    }

/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
    pub async fn generateblock(&self, output: String, transactions: Vec<serde_json::Value>, submit: bool) -> Result<GenerateblockResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(output)?);
        params.push(serde_json::to_value(transactions)?);
        params.push(serde_json::to_value(submit)?);
        // dispatch and deserialize to `GenerateblockResponse`
        self.client.call::<GenerateblockResponse>("generateblock", &params).await
    }

/// Mine to a specified address and return the block hashes.
    pub async fn generatetoaddress(&self, nblocks: u64, address: String, maxtries: u64) -> Result<GeneratetoaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(nblocks)?);
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(maxtries)?);
        // dispatch and deserialize to `GeneratetoaddressResponse`
        self.client.call::<GeneratetoaddressResponse>("generatetoaddress", &params).await
    }

/// Mine to a specified descriptor and return the block hashes.
    pub async fn generatetodescriptor(&self, num_blocks: u64, descriptor: String, maxtries: u64) -> Result<GeneratetodescriptorResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(num_blocks)?);
        params.push(serde_json::to_value(descriptor)?);
        params.push(serde_json::to_value(maxtries)?);
        // dispatch and deserialize to `GeneratetodescriptorResponse`
        self.client.call::<GeneratetodescriptorResponse>("generatetodescriptor", &params).await
    }

/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
    pub async fn getaddednodeinfo(&self, node: String) -> Result<GetaddednodeinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(node)?);
        // dispatch and deserialize to `GetaddednodeinfoResponse`
        self.client.call::<GetaddednodeinfoResponse>("getaddednodeinfo", &params).await
    }

/// Provides information about the node"s address manager by returning the number of addresses in the ``new`` and ``tried`` tables and their sum for all networks.
    pub async fn getaddrmaninfo(&self) -> Result<GetaddrmaninfoResponse, TransportError> {
        // dispatch and deserialize to `GetaddrmaninfoResponse`
        self.client.call::<GetaddrmaninfoResponse>("getaddrmaninfo", &[]).await
    }

/// Returns the hash of the best (tip) block in the most-work fully-validated chain.
    pub async fn getbestblockhash(&self) -> Result<GetbestblockhashResponse, TransportError> {
        // dispatch and deserialize to `GetbestblockhashResponse`
        self.client.call::<GetbestblockhashResponse>("getbestblockhash", &[]).await
    }

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block "hash".
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
    pub async fn getblock(&self, blockhash: bitcoin::BlockHash, verbosity: u32) -> Result<GetblockResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        params.push(serde_json::to_value(verbosity)?);
        // dispatch and deserialize to `GetblockResponse`
        self.client.call::<GetblockResponse>("getblock", &params).await
    }

/// Returns an object containing various state info regarding blockchain processing.
    pub async fn getblockchaininfo(&self) -> Result<GetblockchaininfoResponse, TransportError> {
        // dispatch and deserialize to `GetblockchaininfoResponse`
        self.client.call::<GetblockchaininfoResponse>("getblockchaininfo", &[]).await
    }

/// Returns the height of the most-work fully-validated chain.
/// The genesis block has height 0.
    pub async fn getblockcount(&self) -> Result<GetblockcountResponse, TransportError> {
        // dispatch and deserialize to `GetblockcountResponse`
        self.client.call::<GetblockcountResponse>("getblockcount", &[]).await
    }

/// Retrieve a BIP 157 content filter for a particular block.
    pub async fn getblockfilter(&self, blockhash: bitcoin::BlockHash, filtertype: String) -> Result<GetblockfilterResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        params.push(serde_json::to_value(filtertype)?);
        // dispatch and deserialize to `GetblockfilterResponse`
        self.client.call::<GetblockfilterResponse>("getblockfilter", &params).await
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
    pub async fn getblockfrompeer(&self, blockhash: bitcoin::BlockHash, peer_id: u64) -> Result<GetblockfrompeerResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        params.push(serde_json::to_value(peer_id)?);
        // dispatch and deserialize to `GetblockfrompeerResponse`
        self.client.call::<GetblockfrompeerResponse>("getblockfrompeer", &params).await
    }

/// Returns hash of block in best-block-chain at height provided.
    pub async fn getblockhash(&self, height: u64) -> Result<GetblockhashResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(height)?);
        // dispatch and deserialize to `GetblockhashResponse`
        self.client.call::<GetblockhashResponse>("getblockhash", &params).await
    }

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader "hash".
/// If verbose is true, returns an Object with information about blockheader <hash>.
    pub async fn getblockheader(&self, blockhash: bitcoin::BlockHash, verbose: bool) -> Result<GetblockheaderResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        params.push(serde_json::to_value(verbose)?);
        // dispatch and deserialize to `GetblockheaderResponse`
        self.client.call::<GetblockheaderResponse>("getblockheader", &params).await
    }

/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won"t work for some heights with pruning.
    pub async fn getblockstats(&self, hash_or_height: u64, stats: Vec<serde_json::Value>) -> Result<GetblockstatsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hash_or_height)?);
        params.push(serde_json::to_value(stats)?);
        // dispatch and deserialize to `GetblockstatsResponse`
        self.client.call::<GetblockstatsResponse>("getblockstats", &params).await
    }

/// If the request parameters include a "mode" key, that is used to explicitly select between the default "template" request or a "proposal".
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
/// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
/// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
    pub async fn getblocktemplate(&self, template_request: serde_json::Value) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(template_request)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("getblocktemplate", &params).await
    }

/// Return information about chainstates.
    pub async fn getchainstates(&self) -> Result<GetchainstatesResponse, TransportError> {
        // dispatch and deserialize to `GetchainstatesResponse`
        self.client.call::<GetchainstatesResponse>("getchainstates", &[]).await
    }

/// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
    pub async fn getchaintips(&self) -> Result<GetchaintipsResponse, TransportError> {
        // dispatch and deserialize to `GetchaintipsResponse`
        self.client.call::<GetchaintipsResponse>("getchaintips", &[]).await
    }

/// Compute statistics about the total number and rate of transactions in the chain.
    pub async fn getchaintxstats(&self, nblocks: u64, blockhash: bitcoin::BlockHash) -> Result<GetchaintxstatsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(nblocks)?);
        params.push(serde_json::to_value(blockhash)?);
        // dispatch and deserialize to `GetchaintxstatsResponse`
        self.client.call::<GetchaintxstatsResponse>("getchaintxstats", &params).await
    }

/// Returns the number of connections to other nodes.
    pub async fn getconnectioncount(&self) -> Result<GetconnectioncountResponse, TransportError> {
        // dispatch and deserialize to `GetconnectioncountResponse`
        self.client.call::<GetconnectioncountResponse>("getconnectioncount", &[]).await
    }

/// Returns an object containing various state info regarding deployments of consensus changes.
    pub async fn getdeploymentinfo(&self, blockhash: bitcoin::BlockHash) -> Result<GetdeploymentinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        // dispatch and deserialize to `GetdeploymentinfoResponse`
        self.client.call::<GetdeploymentinfoResponse>("getdeploymentinfo", &params).await
    }

/// Analyses a descriptor.
    pub async fn getdescriptorinfo(&self, descriptor: String) -> Result<GetdescriptorinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(descriptor)?);
        // dispatch and deserialize to `GetdescriptorinfoResponse`
        self.client.call::<GetdescriptorinfoResponse>("getdescriptorinfo", &params).await
    }

/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
    pub async fn getdifficulty(&self) -> Result<GetdifficultyResponse, TransportError> {
        // dispatch and deserialize to `GetdifficultyResponse`
        self.client.call::<GetdifficultyResponse>("getdifficulty", &[]).await
    }

/// List all BIP 32 HD keys in the wallet and which descriptors use them.
    pub async fn gethdkeys(&self, options: serde_json::Value) -> Result<GethdkeysResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(options)?);
        // dispatch and deserialize to `GethdkeysResponse`
        self.client.call::<GethdkeysResponse>("gethdkeys", &params).await
    }

/// Returns the status of one or all available indices currently running in the node.
    pub async fn getindexinfo(&self, index_name: String) -> Result<GetindexinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(index_name)?);
        // dispatch and deserialize to `GetindexinfoResponse`
        self.client.call::<GetindexinfoResponse>("getindexinfo", &params).await
    }

/// Returns an object containing information about memory usage.
    pub async fn getmemoryinfo(&self, mode: String) -> Result<GetmemoryinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(mode)?);
        // dispatch and deserialize to `GetmemoryinfoResponse`
        self.client.call::<GetmemoryinfoResponse>("getmemoryinfo", &params).await
    }

/// If txid is in the mempool, returns all in-mempool ancestors.
    pub async fn getmempoolancestors(&self, txid: bitcoin::Txid, verbose: bool) -> Result<GetmempoolancestorsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(verbose)?);
        // dispatch and deserialize to `GetmempoolancestorsResponse`
        self.client.call::<GetmempoolancestorsResponse>("getmempoolancestors", &params).await
    }

/// If txid is in the mempool, returns all in-mempool descendants.
    pub async fn getmempooldescendants(&self, txid: bitcoin::Txid, verbose: bool) -> Result<GetmempooldescendantsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(verbose)?);
        // dispatch and deserialize to `GetmempooldescendantsResponse`
        self.client.call::<GetmempooldescendantsResponse>("getmempooldescendants", &params).await
    }

/// Returns mempool data for given transaction
    pub async fn getmempoolentry(&self, txid: bitcoin::Txid) -> Result<GetmempoolentryResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        // dispatch and deserialize to `GetmempoolentryResponse`
        self.client.call::<GetmempoolentryResponse>("getmempoolentry", &params).await
    }

/// Returns details on the active state of the TX memory pool.
    pub async fn getmempoolinfo(&self) -> Result<GetmempoolinfoResponse, TransportError> {
        // dispatch and deserialize to `GetmempoolinfoResponse`
        self.client.call::<GetmempoolinfoResponse>("getmempoolinfo", &[]).await
    }

/// Returns a json object containing mining-related information.
    pub async fn getmininginfo(&self) -> Result<GetmininginfoResponse, TransportError> {
        // dispatch and deserialize to `GetmininginfoResponse`
        self.client.call::<GetmininginfoResponse>("getmininginfo", &[]).await
    }

/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.
    pub async fn getnettotals(&self) -> Result<GetnettotalsResponse, TransportError> {
        // dispatch and deserialize to `GetnettotalsResponse`
        self.client.call::<GetnettotalsResponse>("getnettotals", &[]).await
    }

/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.
    pub async fn getnetworkhashps(&self, nblocks: u64, height: u64) -> Result<GetnetworkhashpsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(nblocks)?);
        params.push(serde_json::to_value(height)?);
        // dispatch and deserialize to `GetnetworkhashpsResponse`
        self.client.call::<GetnetworkhashpsResponse>("getnetworkhashps", &params).await
    }

/// Returns an object containing various state info regarding P2P networking.
    pub async fn getnetworkinfo(&self) -> Result<GetnetworkinfoResponse, TransportError> {
        // dispatch and deserialize to `GetnetworkinfoResponse`
        self.client.call::<GetnetworkinfoResponse>("getnetworkinfo", &[]).await
    }

/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.
    pub async fn getnodeaddresses(&self, count: u64, network: String) -> Result<GetnodeaddressesResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(count)?);
        params.push(serde_json::to_value(network)?);
        // dispatch and deserialize to `GetnodeaddressesResponse`
        self.client.call::<GetnodeaddressesResponse>("getnodeaddresses", &params).await
    }

/// Shows transactions in the tx orphanage.
///
/// EXPERIMENTAL warning: this call may be changed in future releases.
    pub async fn getorphantxs(&self, verbosity: u32) -> Result<GetorphantxsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(verbosity)?);
        // dispatch and deserialize to `GetorphantxsResponse`
        self.client.call::<GetorphantxsResponse>("getorphantxs", &params).await
    }

/// Returns data about each connected network peer as a json array of objects.
    pub async fn getpeerinfo(&self) -> Result<GetpeerinfoResponse, TransportError> {
        // dispatch and deserialize to `GetpeerinfoResponse`
        self.client.call::<GetpeerinfoResponse>("getpeerinfo", &[]).await
    }

/// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
    pub async fn getprioritisedtransactions(&self) -> Result<GetprioritisedtransactionsResponse, TransportError> {
        // dispatch and deserialize to `GetprioritisedtransactionsResponse`
        self.client.call::<GetprioritisedtransactionsResponse>("getprioritisedtransactions", &[]).await
    }

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Returns information on all address manager entries for the new and tried tables.
    pub async fn getrawaddrman(&self) -> Result<GetrawaddrmanResponse, TransportError> {
        // dispatch and deserialize to `GetrawaddrmanResponse`
        self.client.call::<GetrawaddrmanResponse>("getrawaddrman", &[]).await
    }

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
///
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
    pub async fn getrawmempool(&self, verbose: bool, mempool_sequence: bool) -> Result<GetrawmempoolResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(verbose)?);
        params.push(serde_json::to_value(mempool_sequence)?);
        // dispatch and deserialize to `GetrawmempoolResponse`
        self.client.call::<GetrawmempoolResponse>("getrawmempool", &params).await
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
    pub async fn getrawtransaction(&self, txid: bitcoin::Txid, verbosity: u32, blockhash: bitcoin::BlockHash) -> Result<GetrawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(verbosity)?);
        params.push(serde_json::to_value(blockhash)?);
        // dispatch and deserialize to `GetrawtransactionResponse`
        self.client.call::<GetrawtransactionResponse>("getrawtransaction", &params).await
    }

/// Returns details of the RPC server.
    pub async fn getrpcinfo(&self) -> Result<GetrpcinfoResponse, TransportError> {
        // dispatch and deserialize to `GetrpcinfoResponse`
        self.client.call::<GetrpcinfoResponse>("getrpcinfo", &[]).await
    }

/// Returns details about an unspent transaction output.
    pub async fn gettxout(&self, txid: bitcoin::Txid, n: u32, include_mempool: bool) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(n)?);
        params.push(serde_json::to_value(include_mempool)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("gettxout", &params).await
    }

/// Returns a hex-encoded proof that "txid" was included in a block.
///
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
    pub async fn gettxoutproof(&self, txids: Vec<bitcoin::Txid>, blockhash: bitcoin::BlockHash) -> Result<GettxoutproofResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txids)?);
        params.push(serde_json::to_value(blockhash)?);
        // dispatch and deserialize to `GettxoutproofResponse`
        self.client.call::<GettxoutproofResponse>("gettxoutproof", &params).await
    }

/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
    pub async fn gettxoutsetinfo(&self, hash_type: String, hash_or_height: u64, use_index: bool) -> Result<GettxoutsetinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hash_type)?);
        params.push(serde_json::to_value(hash_or_height)?);
        params.push(serde_json::to_value(use_index)?);
        // dispatch and deserialize to `GettxoutsetinfoResponse`
        self.client.call::<GettxoutsetinfoResponse>("gettxoutsetinfo", &params).await
    }

/// Scans the mempool to find transactions spending any of the given outputs
    pub async fn gettxspendingprevout(&self, outputs: Vec<serde_json::Value>) -> Result<GettxspendingprevoutResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(outputs)?);
        // dispatch and deserialize to `GettxspendingprevoutResponse`
        self.client.call::<GettxspendingprevoutResponse>("gettxspendingprevout", &params).await
    }

/// List all commands, or get help for a specified command.
    pub async fn help(&self, command: String) -> Result<HelpResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(command)?);
        // dispatch and deserialize to `HelpResponse`
        self.client.call::<HelpResponse>("help", &params).await
    }

/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
    pub async fn importmempool(&self, filepath: String, options: serde_json::Value) -> Result<ImportmempoolResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(filepath)?);
        params.push(serde_json::to_value(options)?);
        // dispatch and deserialize to `ImportmempoolResponse`
        self.client.call::<ImportmempoolResponse>("importmempool", &params).await
    }

/// Permanently marks a block as invalid, as if it violated a consensus rule.
    pub async fn invalidateblock(&self, blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("invalidateblock", &params).await
    }

/// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
/// No input in any of the PSBTs can be in more than one of the PSBTs.
    pub async fn joinpsbts(&self, txs: Vec<serde_json::Value>) -> Result<JoinpsbtsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txs)?);
        // dispatch and deserialize to `JoinpsbtsResponse`
        self.client.call::<JoinpsbtsResponse>("joinpsbts", &params).await
    }

/// List all manually banned IPs/Subnets.
    pub async fn listbanned(&self) -> Result<ListbannedResponse, TransportError> {
        // dispatch and deserialize to `ListbannedResponse`
        self.client.call::<ListbannedResponse>("listbanned", &[]).await
    }

/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network"s tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
///
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
///
/// You can find more information on this process in the ``assumeutxo`` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
    pub async fn loadtxoutset(&self, path: String) -> Result<LoadtxoutsetResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(path)?);
        // dispatch and deserialize to `LoadtxoutsetResponse`
        self.client.call::<LoadtxoutsetResponse>("loadtxoutset", &params).await
    }

/// Gets and sets the logging configuration.
/// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
/// When called with arguments, adds or removes categories from debug logging and return the lists above.
/// The arguments are evaluated in order "include", "exclude".
/// If an item is both included and excluded, it will thus end up being excluded.
/// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
/// In addition, the following are available as category names with special meanings:
/// - "all",  "1" : represent all logging categories.
    pub async fn logging(&self, include: Vec<serde_json::Value>, exclude: Vec<serde_json::Value>) -> Result<LoggingResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(include)?);
        params.push(serde_json::to_value(exclude)?);
        // dispatch and deserialize to `LoggingResponse`
        self.client.call::<LoggingResponse>("logging", &params).await
    }

/// Bump the scheduler into the future (-regtest only)
    pub async fn mockscheduler(&self, delta_time: u64) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(delta_time)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("mockscheduler", &params).await
    }

/// Requests that a ping be sent to all other nodes, to measure ping time.
/// Results provided in getpeerinfo, pingtime and pingwait fields are decimal seconds.
/// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
    pub async fn ping(&self) -> Result<(), TransportError> {
        // dispatch and deserialize to `()`
        self.client.call::<()>("ping", &[]).await
    }

/// Treats a block as if it were received before others with the same work.
///
/// A later preciousblock call can override the effect of an earlier one.
///
/// The effects of preciousblock are not retained across restarts.
    pub async fn preciousblock(&self, blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("preciousblock", &params).await
    }

/// Accepts the transaction into mined blocks at a higher (or lower) priority
    pub async fn prioritisetransaction(&self, txid: bitcoin::Txid, dummy: Option<String>, fee_delta: f64) -> Result<PrioritisetransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(dummy)?);
        params.push(serde_json::to_value(fee_delta)?);
        // dispatch and deserialize to `PrioritisetransactionResponse`
        self.client.call::<PrioritisetransactionResponse>("prioritisetransaction", &params).await
    }


    pub async fn pruneblockchain(&self, height: u64) -> Result<PruneblockchainResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(height)?);
        // dispatch and deserialize to `PruneblockchainResponse`
        self.client.call::<PruneblockchainResponse>("pruneblockchain", &params).await
    }

/// Removes invalidity status of a block, its ancestors and its descendants, reconsider them for activation.
/// This can be used to undo the effects of invalidateblock.
    pub async fn reconsiderblock(&self, blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("reconsiderblock", &params).await
    }

/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
    pub async fn savemempool(&self) -> Result<SavemempoolResponse, TransportError> {
        // dispatch and deserialize to `SavemempoolResponse`
        self.client.call::<SavemempoolResponse>("savemempool", &[]).await
    }

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn scanblocks(&self, action: String, scanobjects: Vec<serde_json::Value>, start_height: u64, stop_height: u64, filtertype: String, options: serde_json::Value) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(action)?);
        params.push(serde_json::to_value(scanobjects)?);
        params.push(serde_json::to_value(start_height)?);
        params.push(serde_json::to_value(stop_height)?);
        params.push(serde_json::to_value(filtertype)?);
        params.push(serde_json::to_value(options)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("scanblocks", &params).await
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
    pub async fn scantxoutset(&self, action: String, scanobjects: Vec<serde_json::Value>) -> Result<ScantxoutsetResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(action)?);
        params.push(serde_json::to_value(scanobjects)?);
        // dispatch and deserialize to `ScantxoutsetResponse`
        self.client.call::<ScantxoutsetResponse>("scantxoutset", &params).await
    }

/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.
    pub async fn sendmsgtopeer(&self, peer_id: u64, msg_type: String, msg: String) -> Result<SendmsgtopeerResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(peer_id)?);
        params.push(serde_json::to_value(msg_type)?);
        params.push(serde_json::to_value(msg)?);
        // dispatch and deserialize to `SendmsgtopeerResponse`
        self.client.call::<SendmsgtopeerResponse>("sendmsgtopeer", &params).await
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
    pub async fn sendrawtransaction(&self, hexstring: String, maxfeerate: f64, maxburnamount: f64) -> Result<SendrawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        params.push(serde_json::to_value(maxfeerate)?);
        params.push(serde_json::to_value(maxburnamount)?);
        // dispatch and deserialize to `SendrawtransactionResponse`
        self.client.call::<SendrawtransactionResponse>("sendrawtransaction", &params).await
    }

/// Attempts to add or remove an IP/Subnet from the banned list.
    pub async fn setban(&self, subnet: String, command: String, bantime: u64, absolute: bool) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(subnet)?);
        params.push(serde_json::to_value(command)?);
        params.push(serde_json::to_value(bantime)?);
        params.push(serde_json::to_value(absolute)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("setban", &params).await
    }

/// Set the local time to given timestamp (-regtest only)
    pub async fn setmocktime(&self, timestamp: u64) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(timestamp)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("setmocktime", &params).await
    }

/// Disable/enable all p2p network activity.
    pub async fn setnetworkactive(&self, state: bool) -> Result<SetnetworkactiveResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(state)?);
        // dispatch and deserialize to `SetnetworkactiveResponse`
        self.client.call::<SetnetworkactiveResponse>("setnetworkactive", &params).await
    }

/// Sign a message with the private key of an address
    pub async fn signmessagewithprivkey(&self, privkey: String, message: String) -> Result<SignmessagewithprivkeyResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(privkey)?);
        params.push(serde_json::to_value(message)?);
        // dispatch and deserialize to `SignmessagewithprivkeyResponse`
        self.client.call::<SignmessagewithprivkeyResponse>("signmessagewithprivkey", &params).await
    }

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
    pub async fn signrawtransactionwithkey(&self, hexstring: String, privkeys: Vec<String>, prevtxs: Vec<serde_json::Value>, sighashtype: String) -> Result<SignrawtransactionwithkeyResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        params.push(serde_json::to_value(privkeys)?);
        params.push(serde_json::to_value(prevtxs)?);
        params.push(serde_json::to_value(sighashtype)?);
        // dispatch and deserialize to `SignrawtransactionwithkeyResponse`
        self.client.call::<SignrawtransactionwithkeyResponse>("signrawtransactionwithkey", &params).await
    }

/// Request a graceful shutdown of Bitcoin Core.
    pub async fn stop(&self, wait: u64) -> Result<StopResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(wait)?);
        // dispatch and deserialize to `StopResponse`
        self.client.call::<StopResponse>("stop", &params).await
    }

/// Attempts to submit new block to network.
/// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
    pub async fn submitblock(&self, hexdata: String, dummy: Option<String>) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexdata)?);
        params.push(serde_json::to_value(dummy)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("submitblock", &params).await
    }

/// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
/// Throws when the header is invalid.
    pub async fn submitheader(&self, hexdata: String) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexdata)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("submitheader", &params).await
    }

/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
    pub async fn submitpackage(&self, package: Vec<serde_json::Value>, maxfeerate: f64, maxburnamount: f64) -> Result<SubmitpackageResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(package)?);
        params.push(serde_json::to_value(maxfeerate)?);
        params.push(serde_json::to_value(maxburnamount)?);
        // dispatch and deserialize to `SubmitpackageResponse`
        self.client.call::<SubmitpackageResponse>("submitpackage", &params).await
    }

/// Waits for the validation interface queue to catch up on everything that was there when we entered this function.
    pub async fn syncwithvalidationinterfacequeue(&self) -> Result<(), TransportError> {
        // dispatch and deserialize to `()`
        self.client.call::<()>("syncwithvalidationinterfacequeue", &[]).await
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
    pub async fn testmempoolaccept(&self, rawtxs: Vec<serde_json::Value>, maxfeerate: f64) -> Result<TestmempoolacceptResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(rawtxs)?);
        params.push(serde_json::to_value(maxfeerate)?);
        // dispatch and deserialize to `TestmempoolacceptResponse`
        self.client.call::<TestmempoolacceptResponse>("testmempoolaccept", &params).await
    }

/// Returns the total uptime of the server.
    pub async fn uptime(&self) -> Result<UptimeResponse, TransportError> {
        // dispatch and deserialize to `UptimeResponse`
        self.client.call::<UptimeResponse>("uptime", &[]).await
    }

/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
    pub async fn utxoupdatepsbt(&self, psbt: String, descriptors: Vec<serde_json::Value>) -> Result<UtxoupdatepsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(psbt)?);
        params.push(serde_json::to_value(descriptors)?);
        // dispatch and deserialize to `UtxoupdatepsbtResponse`
        self.client.call::<UtxoupdatepsbtResponse>("utxoupdatepsbt", &params).await
    }

/// Return information about the given bitcoin address.
    pub async fn validateaddress(&self, address: String) -> Result<ValidateaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        // dispatch and deserialize to `ValidateaddressResponse`
        self.client.call::<ValidateaddressResponse>("validateaddress", &params).await
    }

/// Verifies blockchain database.
    pub async fn verifychain(&self, checklevel: u32, nblocks: u64) -> Result<VerifychainResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(checklevel)?);
        params.push(serde_json::to_value(nblocks)?);
        // dispatch and deserialize to `VerifychainResponse`
        self.client.call::<VerifychainResponse>("verifychain", &params).await
    }

/// Verify a signed message.
    pub async fn verifymessage(&self, address: String, signature: String, message: String) -> Result<VerifymessageResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(signature)?);
        params.push(serde_json::to_value(message)?);
        // dispatch and deserialize to `VerifymessageResponse`
        self.client.call::<VerifymessageResponse>("verifymessage", &params).await
    }

/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
    pub async fn verifytxoutproof(&self, proof: String) -> Result<VerifytxoutproofResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(proof)?);
        // dispatch and deserialize to `VerifytxoutproofResponse`
        self.client.call::<VerifytxoutproofResponse>("verifytxoutproof", &params).await
    }

/// Waits for a specific new block and returns useful info about it.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn waitforblock(&self, blockhash: bitcoin::BlockHash, timeout: u64) -> Result<WaitforblockResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        params.push(serde_json::to_value(timeout)?);
        // dispatch and deserialize to `WaitforblockResponse`
        self.client.call::<WaitforblockResponse>("waitforblock", &params).await
    }

/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn waitforblockheight(&self, height: u64, timeout: u64) -> Result<WaitforblockheightResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(height)?);
        params.push(serde_json::to_value(timeout)?);
        // dispatch and deserialize to `WaitforblockheightResponse`
        self.client.call::<WaitforblockheightResponse>("waitforblockheight", &params).await
    }

/// Waits for any new block and returns useful info about it.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn waitfornewblock(&self, timeout: u64) -> Result<WaitfornewblockResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(timeout)?);
        // dispatch and deserialize to `WaitfornewblockResponse`
        self.client.call::<WaitfornewblockResponse>("waitfornewblock", &params).await
    }
}

