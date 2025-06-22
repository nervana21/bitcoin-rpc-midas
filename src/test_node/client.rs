use anyhow::Result;
use std::sync::Arc;
use crate::transport::core::{TransportError};
use crate::transport::{DefaultTransport, RpcClient, BatchBuilder};
use crate::types::v28_types::*;
use serde_json::Value;

use crate::node::{BitcoinNodeManager, TestConfig};

use super::node::BitcoinNodeClient;
use super::wallet::BitcoinWalletClient;

use bitcoin::Amount;
/// Trait for managing a Bitcoin node's lifecycle
pub trait NodeManager: Send + Sync + std::fmt::Debug + std::any::Any {
    fn start(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>>;
    fn stop(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>>;
    fn rpc_port(&self) -> u16;
    fn as_any(&self) -> &dyn std::any::Any;
}

#[derive(Debug)]
pub struct BitcoinTestClient {
node_client: BitcoinNodeClient,
wallet_client: BitcoinWalletClient,
node_manager: Option<Box<dyn NodeManager>>,
/// A thin RPC wrapper around the transport, with batching built in
rpc: RpcClient,
}

impl NodeManager for BitcoinNodeManager {
fn start(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>> {
println!("[DEBUG] NodeManager::start called on BitcoinNodeManager");
Box::pin(async move {
println!("[DEBUG] Inside NodeManager::start async block");
let result = self.start_internal().await;
println!("[DEBUG] NodeManager::start result: {:?}", result);
result.map_err(|e| TransportError::Rpc(e.to_string()))
})
}

fn stop(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>> {
println!("[DEBUG] NodeManager::stop called on BitcoinNodeManager");
Box::pin(async move {
println!("[DEBUG] Inside NodeManager::stop async block");
let result = self.stop_internal().await;
println!("[DEBUG] NodeManager::stop result: {:?}", result);
result.map_err(|e| TransportError::Rpc(e.to_string()))
})
}

fn rpc_port(&self) -> u16 {
println!("[DEBUG] NodeManager::rpc_port called on BitcoinNodeManager");
self.rpc_port
}

fn as_any(&self) -> &dyn std::any::Any {
println!("[DEBUG] NodeManager::as_any called on BitcoinNodeManager");
self
}
}

/// Options for creating or loading a Bitcoin Core wallet
#[derive(Debug, Clone)]
pub struct WalletOptions {
    pub disable_private_keys: bool,
    pub blank: bool,
    pub passphrase: String,
    pub avoid_reuse: bool,
    pub descriptors: bool,
    pub load_on_startup: bool,
    pub external_signer: bool,
}

impl Default for WalletOptions {
    fn default() -> Self {
        WalletOptions {
            disable_private_keys: false,
            blank: false,
            passphrase: "".to_string(),
            avoid_reuse: false,
            descriptors: false,
            load_on_startup: false,
            external_signer: false,
        }
    }
}

impl WalletOptions {
    pub fn with_descriptors(mut self) -> Self {
        self.descriptors = true;
        self
    }
}

impl BitcoinTestClient {
    pub async fn new() -> Result<Self, TransportError> {
        println!("[DEBUG] BitcoinTestClient::new called");
        let config = TestConfig::default();
        let node_manager = BitcoinNodeManager::new_with_config(&config)?;
        Self::new_with_manager(node_manager).await
    }

    /// Creates a new Bitcoin test client with a specific node manager.
    /// This allows for custom node configuration and lifecycle management.
    /// The node manager must implement the `NodeManager` trait.
    /// ```
    pub async fn new_with_manager<M: NodeManager + 'static>(mut node_manager: M) -> Result<Self, TransportError> {
        println!("[DEBUG] BitcoinTestClient::new_with_manager called");
        // Start the node
        println!("[DEBUG] Calling node_manager.start()");
        node_manager.start().await?;
        println!("[DEBUG] node_manager.start() completed successfully");
        
        // Wait for node to be ready for RPC
        println!("[DEBUG] Creating transport with port {}", node_manager.rpc_port());
        let transport = Arc::new(DefaultTransport::new(
&format!("http://127.0.0.1:{}", node_manager.rpc_port()),
            Some(("rpcuser".to_string(), "rpcpassword".to_string())),
        ));
        
        // Create RPC client for batching support
        let rpc = RpcClient::from_transport(transport.clone());
        
        // Create node and wallet clients
        let node_client = BitcoinNodeClient::new(transport.clone());
        
        // Wait for node to be ready for RPC
        // Core initialization states that require waiting:
        // -28: RPC in warmup
        // -4:  RPC in warmup (alternative code)
        let init_states = [
            "\"code\":-28",
            "\"code\":-4",
        ];
        
        let max_retries = 30;
        let mut retries = 0;
        
        loop {
            match node_client.getblockchaininfo().await {
                Ok(_) => break,
                Err(TransportError::Rpc(e)) => {
                    // Check if the error matches any known initialization state
                    let is_init_state = init_states.iter().any(|state| e.contains(state));
                    if is_init_state && retries < max_retries {
                        println!("[DEBUG] Waiting for initialization: {} (attempt {}/{})", e, retries + 1, max_retries);
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        retries += 1;
                        continue;
                    }
                    return Err(TransportError::Rpc(e));
                }
                Err(e) => return Err(e),
            }
        }
        
        if retries > 0 {
            println!("[DEBUG] Node initialization completed after {} attempts", retries);
        }
        
        Ok(Self {
            node_client,
            wallet_client: BitcoinWalletClient::new(transport.clone()),
            node_manager: Some(Box::new(node_manager)),
            rpc,
        })
    }
    /// Ensures a wallet exists using the given options.
/// Loads the wallet if it already exists. Returns the wallet name.
pub async fn ensure_wallet_with_options(
&mut self,
wallet_name: impl Into<String>,
opts: WalletOptions,
) -> Result<String, TransportError> {
let wallet_name = wallet_name.into();

// Check if wallet is currently loaded
let wallets = self.wallet_client.listwallets().await?;
if wallets.0.iter().any(|w| w == &wallet_name) {
self.wallet_client.unloadwallet(wallet_name.clone(), false).await?;
}

// Try to create wallet
match self.wallet_client
.createwallet(
wallet_name.clone(),
opts.disable_private_keys,
opts.blank,
opts.passphrase.clone(),
opts.avoid_reuse,
opts.descriptors,
opts.load_on_startup,
opts.external_signer,
)
.await
{
Ok(_) => Ok(wallet_name),
Err(TransportError::Rpc(err)) if err.contains("\"code\":-4") => {
// Try loading instead
self.wallet_client.loadwallet(wallet_name.clone(), false).await?;

let new_transport = Arc::new(
DefaultTransport::new(
&format!("http://127.0.0.1:{}", self.node_manager.as_ref().unwrap().rpc_port()),
Some(("rpcuser".to_string(), "rpcpassword".to_string())),
)
.with_wallet(wallet_name.clone())
);

self.wallet_client.with_transport(new_transport.clone());
self.node_client.with_transport(new_transport);

Ok(wallet_name)
},
Err(e) => Err(e),
}
}

/// Shortcut for `ensure_wallet_with_options("test_wallet", WalletOptions::default().with_descriptors())`
pub async fn ensure_default_wallet(&mut self, name: impl Into<String>) -> Result<String, TransportError> {
self.ensure_wallet_with_options(name, WalletOptions::default().with_descriptors()).await
}

    /// Helper method to mine blocks to a new address
    pub async fn mine_blocks(&mut self, num_blocks: u64, maxtries: u64) -> Result<(String, Value), TransportError> {
        // Ensure we have a wallet with default settings
        let _wallet_name = self.ensure_default_wallet("test_wallet").await?;

        println!("[debug] Getting new address");
        let address = self.wallet_client.getnewaddress("".to_string(), "bech32m".to_string()).await?;
        println!("[debug] Generated address: {:?}", address);
        println!("[debug] Generating blocks");
        let blocks = self.node_client.generatetoaddress(
            num_blocks,
            address.0.clone(),
            maxtries
        ).await?;
        println!("[debug] Generated blocks: {:?}", blocks);
        Ok((address.0, serde_json::to_value(blocks)?))
    }

    /// Resets the blockchain to a clean state.
/// This method:
/// 1. First attempts to prune the blockchain to height 0
/// 2. If blocks remain, invalidates all blocks except genesis
/// 3. Reconsiders the genesis block to maintain a valid chain
pub async fn reset_chain(&mut self) -> Result<(), TransportError> {
// First try pruning to height 0
self.node_client.pruneblockchain(0).await?;
// Check if we still have blocks
let info = self.node_client.getblockchaininfo().await?;
let current_height = info.blocks;
if current_height > 1 {
// Invalidate all blocks except genesis
for height in (1..=current_height).rev() {
let block_hash = self.node_client.getblockhash(height).await?.0;
self.node_client.invalidateblock(block_hash).await?;
}
// Reconsider genesis block
let genesis_hash = self.node_client.getblockhash(0).await?.0;
self.node_client.reconsiderblock(genesis_hash).await?;
}
Ok(())
}

    /// Stops the Bitcoin node if one is running.
/// This is automatically called when the client is dropped.
pub async fn stop_node(&mut self) -> Result<(), TransportError> {
if let Some(mut manager) = self.node_manager.take() {
manager.stop().await?;
}
Ok(())
}

    /// Returns a reference to the node manager if one exists.
/// This can be used to access node configuration and control the node lifecycle.
pub fn node_manager(&self) -> Option<&dyn NodeManager> {
self.node_manager.as_deref()
}

    /// Give callers the full RPC client (incl. `.batch()`)
pub fn rpc(&self) -> &RpcClient {
&self.rpc
}

    /// Begin a JSON-RPC batch against this test node
    pub fn batch(&self) -> BatchBuilder {
        self.rpc.batch()
    }

/// Mark in-wallet transaction <txid> as abandoned
/// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
/// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
/// It only works on transactions which are not included in a block and are not currently in the mempool.
/// It has no effect on transactions which are already abandoned.
    pub async fn abandontransaction(&self, txid: bitcoin::Txid) -> Result<(), TransportError> {
        self.wallet_client.abandontransaction(txid).await
    }

/// Stops current wallet rescan triggered by an RPC call, e.g. by an importprivkey call.
/// Note: Use "getwalletinfo" to query the scanning progress.
    pub async fn abortrescan(&self) -> Result<AbortrescanResponse, TransportError> {
        self.wallet_client.abortrescan().await
    }

/// Open an outbound connection to a specified node. This RPC is for testing only.
    pub async fn addconnection(&self, address: String, connection_type: String, v2transport: bool) -> Result<AddconnectionResponse, TransportError> {
        self.node_client.addconnection(address, connection_type, v2transport).await
    }

/// Add an nrequired-to-sign multisignature address to the wallet. Requires a new wallet backup.
/// Each key is a Bitcoin address or hex-encoded public key.
/// This functionality is only intended for use with non-watchonly addresses.
/// See ``importaddress`` for watchonly p2sh address support.
/// If "label" is specified, assign address to that label.
/// Note: This command is only compatible with legacy wallets.
    pub async fn addmultisigaddress(&self, nrequired: u32, keys: Vec<String>, label: String, address_type: String) -> Result<AddmultisigaddressResponse, TransportError> {
        self.wallet_client.addmultisigaddress(nrequired, keys, label, address_type).await
    }

/// Attempts to add or remove a node from the addnode list.
/// Or try a connection to a node once.
/// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
/// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
/// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
    pub async fn addnode(&self, node: String, command: String, v2transport: bool) -> Result<(), TransportError> {
        self.node_client.addnode(node, command, v2transport).await
    }

/// Add the address of a potential peer to an address manager table. This RPC is for testing only.
    pub async fn addpeeraddress(&self, address: String, port: u16, tried: bool) -> Result<AddpeeraddressResponse, TransportError> {
        self.node_client.addpeeraddress(address, port, tried).await
    }

/// Analyzes and provides information about the current status of a PSBT and its inputs
    pub async fn analyzepsbt(&self, psbt: String) -> Result<AnalyzepsbtResponse, TransportError> {
        self.node_client.analyzepsbt(psbt).await
    }

/// Return JSON description of RPC API.
    pub async fn api(&self) -> Result<ApiResponse, TransportError> {
        self.node_client.api().await
    }

/// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
    pub async fn backupwallet(&self, destination: String) -> Result<(), TransportError> {
        self.wallet_client.backupwallet(destination).await
    }

/// Bumps the fee of an opt-in-RBF transaction T, replacing it with a new transaction B.
/// An opt-in RBF transaction with the given txid must be in the wallet.
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
    pub async fn bumpfee(&self, txid: bitcoin::Txid, options: serde_json::Value) -> Result<BumpfeeResponse, TransportError> {
        self.wallet_client.bumpfee(txid, options).await
    }

/// Clear all banned IPs.
    pub async fn clearbanned(&self) -> Result<(), TransportError> {
        self.node_client.clearbanned().await
    }

/// Combine multiple partially signed Bitcoin transactions into one transaction.
/// Implements the Combiner role.
    pub async fn combinepsbt(&self, txs: Vec<serde_json::Value>) -> Result<CombinepsbtResponse, TransportError> {
        self.node_client.combinepsbt(txs).await
    }

/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.
    pub async fn combinerawtransaction(&self, txs: Vec<serde_json::Value>) -> Result<CombinerawtransactionResponse, TransportError> {
        self.node_client.combinerawtransaction(txs).await
    }

/// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// createpsbt and walletcreatefundedpsbt should be used for new applications.
    pub async fn converttopsbt(&self, hexstring: String, permitsigdata: bool, iswitness: bool) -> Result<ConverttopsbtResponse, TransportError> {
        self.node_client.converttopsbt(hexstring, permitsigdata, iswitness).await
    }

/// Creates a multi-signature address with n signature of m keys required.
/// It returns a json object with the address and redeemScript.
    pub async fn createmultisig(&self, nrequired: u32, keys: Vec<String>, address_type: String) -> Result<CreatemultisigResponse, TransportError> {
        self.node_client.createmultisig(nrequired, keys, address_type).await
    }

/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
    pub async fn createpsbt(&self, inputs: Vec<serde_json::Value>, outputs: Vec<serde_json::Value>, locktime: u32, replaceable: bool) -> Result<CreatepsbtResponse, TransportError> {
        self.node_client.createpsbt(inputs, outputs, locktime, replaceable).await
    }

/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction"s inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
    pub async fn createrawtransaction(&self, inputs: Vec<serde_json::Value>, outputs: Vec<serde_json::Value>, locktime: u32, replaceable: bool) -> Result<CreaterawtransactionResponse, TransportError> {
        self.node_client.createrawtransaction(inputs, outputs, locktime, replaceable).await
    }

/// Creates and loads a new wallet.
    pub async fn createwallet(&self, wallet_name: String, disable_private_keys: bool, blank: bool, passphrase: String, avoid_reuse: bool, descriptors: bool, load_on_startup: bool, external_signer: bool) -> Result<CreatewalletResponse, TransportError> {
        self.wallet_client.createwallet(wallet_name, disable_private_keys, blank, passphrase, avoid_reuse, descriptors, load_on_startup, external_signer).await
    }

/// Creates the wallet"s descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn createwalletdescriptor(&self, _type: String, options: serde_json::Value) -> Result<CreatewalletdescriptorResponse, TransportError> {
        self.node_client.createwalletdescriptor(_type, options).await
    }

/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
    pub async fn decodepsbt(&self, psbt: String) -> Result<DecodepsbtResponse, TransportError> {
        self.node_client.decodepsbt(psbt).await
    }

/// Return a JSON object representing the serialized, hex-encoded transaction.
    pub async fn decoderawtransaction(&self, hexstring: String, iswitness: bool) -> Result<DecoderawtransactionResponse, TransportError> {
        self.node_client.decoderawtransaction(hexstring, iswitness).await
    }

/// Decode a hex-encoded script.
    pub async fn decodescript(&self, hexstring: String) -> Result<DecodescriptResponse, TransportError> {
        self.node_client.decodescript(hexstring).await
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
        self.node_client.deriveaddresses(descriptor, range).await
    }

/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
    pub async fn descriptorprocesspsbt(&self, psbt: String, descriptors: Vec<serde_json::Value>, sighashtype: String, bip32derivs: bool, finalize: bool) -> Result<DescriptorprocesspsbtResponse, TransportError> {
        self.node_client.descriptorprocesspsbt(psbt, descriptors, sighashtype, bip32derivs, finalize).await
    }

/// Immediately disconnects from the specified peer node.
///
/// Strictly one out of "address" and "nodeid" can be provided to identify the node.
///
/// To disconnect by nodeid, either set "address" to the empty string, or call using the named "nodeid" argument only.
    pub async fn disconnectnode(&self, address: String, nodeid: u64) -> Result<(), TransportError> {
        self.node_client.disconnectnode(address, nodeid).await
    }

/// Reveals the private key corresponding to "address".
/// Then the importprivkey can be used with this output
/// Note: This command is only compatible with legacy wallets.
    pub async fn dumpprivkey(&self, address: String) -> Result<DumpprivkeyResponse, TransportError> {
        self.wallet_client.dumpprivkey(address).await
    }

/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
///
/// Unless the the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
///
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn dumptxoutset(&self, path: String, _type: String, options: serde_json::Value) -> Result<DumptxoutsetResponse, TransportError> {
        self.node_client.dumptxoutset(path, _type, options).await
    }

/// Dumps all wallet keys in a human-readable format to a server-side file. This does not allow overwriting existing files.
/// Imported scripts are included in the dumpfile, but corresponding BIP173 addresses, etc. may not be added automatically by importwallet.
/// Note that if your wallet contains keys which are not derived from your HD seed (e.g. imported keys), these are not covered by
/// only backing up the seed itself, and must be backed up too (e.g. ensure you back up the whole dumpfile).
/// Note: This command is only compatible with legacy wallets.
    pub async fn dumpwallet(&self, filename: String) -> Result<DumpwalletResponse, TransportError> {
        self.wallet_client.dumpwallet(filename).await
    }

/// Simply echo back the input arguments. This command is for testing.
///
/// It will return an internal bug report when arg9="trigger_internal_bug" is passed.
///
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    pub async fn echo(&self, arg0: String, arg1: String, arg2: String, arg3: String, arg4: String, arg5: String, arg6: String, arg7: String, arg8: String, arg9: String) -> Result<EchoResponse, TransportError> {
        self.node_client.echo(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9).await
    }

/// Echo back the input argument, passing it through a spawned process in a multiprocess build.
/// This command is for testing.
    pub async fn echoipc(&self, arg: String) -> Result<EchoipcResponse, TransportError> {
        self.node_client.echoipc(arg).await
    }

/// Simply echo back the input arguments. This command is for testing.
///
/// It will return an internal bug report when arg9="trigger_internal_bug" is passed.
///
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    pub async fn echojson(&self, arg0: String, arg1: String, arg2: String, arg3: String, arg4: String, arg5: String, arg6: String, arg7: String, arg8: String, arg9: String) -> Result<EchojsonResponse, TransportError> {
        self.node_client.echojson(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9).await
    }

/// Encrypts the wallet with "passphrase". This is for first time encryption.
/// After this, any calls that interact with private keys such as sending or signing
/// will require the passphrase to be set prior the making these calls.
/// Use the walletpassphrase call for this, and then walletlock call.
/// If the wallet is already encrypted, use the walletpassphrasechange call.
/// ** IMPORTANT **
/// For security reasons, the encryption process will generate a new HD seed, resulting
/// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
/// securely back up the newly generated wallet file using the backupwallet RPC.
    pub async fn encryptwallet(&self, passphrase: String) -> Result<EncryptwalletResponse, TransportError> {
        self.wallet_client.encryptwallet(passphrase).await
    }

/// Returns a list of external signers from -signer.
    pub async fn enumeratesigners(&self) -> Result<EnumeratesignersResponse, TransportError> {
        self.node_client.enumeratesigners().await
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
        self.node_client.estimaterawfee(conf_target, threshold).await
    }

/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).
    pub async fn estimatesmartfee(&self, conf_target: u64, estimate_mode: String) -> Result<EstimatesmartfeeResponse, TransportError> {
        self.node_client.estimatesmartfee(conf_target, estimate_mode).await
    }

/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptWitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
    pub async fn finalizepsbt(&self, psbt: String, extract: bool) -> Result<FinalizepsbtResponse, TransportError> {
        self.node_client.finalizepsbt(psbt, extract).await
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
        self.node_client.fundrawtransaction(hexstring, options, iswitness).await
    }

/// has been replaced by the -generate cli option. Refer to -help for more information.
    pub async fn generate(&self) -> Result<(), TransportError> {
        self.node_client.generate().await
    }

/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
    pub async fn generateblock(&self, output: String, transactions: Vec<serde_json::Value>, submit: bool) -> Result<GenerateblockResponse, TransportError> {
        self.node_client.generateblock(output, transactions, submit).await
    }

/// Mine to a specified address and return the block hashes.
    pub async fn generatetoaddress(&self, nblocks: u64, address: String, maxtries: u64) -> Result<GeneratetoaddressResponse, TransportError> {
        self.node_client.generatetoaddress(nblocks, address, maxtries).await
    }

/// Mine to a specified descriptor and return the block hashes.
    pub async fn generatetodescriptor(&self, num_blocks: u64, descriptor: String, maxtries: u64) -> Result<GeneratetodescriptorResponse, TransportError> {
        self.node_client.generatetodescriptor(num_blocks, descriptor, maxtries).await
    }

/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
    pub async fn getaddednodeinfo(&self, node: String) -> Result<GetaddednodeinfoResponse, TransportError> {
        self.node_client.getaddednodeinfo(node).await
    }

/// Returns the list of addresses assigned the specified label.
    pub async fn getaddressesbylabel(&self, label: String) -> Result<GetaddressesbylabelResponse, TransportError> {
        self.wallet_client.getaddressesbylabel(label).await
    }

/// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
    pub async fn getaddressinfo(&self, address: String) -> Result<GetaddressinfoResponse, TransportError> {
        self.wallet_client.getaddressinfo(address).await
    }

/// Provides information about the node"s address manager by returning the number of addresses in the ``new`` and ``tried`` tables and their sum for all networks.
    pub async fn getaddrmaninfo(&self) -> Result<GetaddrmaninfoResponse, TransportError> {
        self.node_client.getaddrmaninfo().await
    }

/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
    pub async fn getbalance(&self, dummy: Option<String>, minconf: u32, include_watchonly: bool, avoid_reuse: bool) -> Result<GetbalanceResponse, TransportError> {
        self.wallet_client.getbalance(dummy, minconf, include_watchonly, avoid_reuse).await
    }

/// Returns an object with all balances in BTC.
    pub async fn getbalances(&self) -> Result<GetbalancesResponse, TransportError> {
        self.wallet_client.getbalances().await
    }

/// Returns the hash of the best (tip) block in the most-work fully-validated chain.
    pub async fn getbestblockhash(&self) -> Result<GetbestblockhashResponse, TransportError> {
        self.node_client.getbestblockhash().await
    }

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block "hash".
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
    pub async fn getblock(&self, blockhash: bitcoin::BlockHash, verbosity: u32) -> Result<GetblockResponse, TransportError> {
        self.node_client.getblock(blockhash, verbosity).await
    }

/// Returns an object containing various state info regarding blockchain processing.
    pub async fn getblockchaininfo(&self) -> Result<GetblockchaininfoResponse, TransportError> {
        self.node_client.getblockchaininfo().await
    }

/// Returns the height of the most-work fully-validated chain.
/// The genesis block has height 0.
    pub async fn getblockcount(&self) -> Result<GetblockcountResponse, TransportError> {
        self.node_client.getblockcount().await
    }

/// Retrieve a BIP 157 content filter for a particular block.
    pub async fn getblockfilter(&self, blockhash: bitcoin::BlockHash, filtertype: String) -> Result<GetblockfilterResponse, TransportError> {
        self.node_client.getblockfilter(blockhash, filtertype).await
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
        self.node_client.getblockfrompeer(blockhash, peer_id).await
    }

/// Returns hash of block in best-block-chain at height provided.
    pub async fn getblockhash(&self, height: u64) -> Result<GetblockhashResponse, TransportError> {
        self.node_client.getblockhash(height).await
    }

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader "hash".
/// If verbose is true, returns an Object with information about blockheader <hash>.
    pub async fn getblockheader(&self, blockhash: bitcoin::BlockHash, verbose: bool) -> Result<GetblockheaderResponse, TransportError> {
        self.node_client.getblockheader(blockhash, verbose).await
    }

/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won"t work for some heights with pruning.
    pub async fn getblockstats(&self, hash_or_height: u64, stats: Vec<serde_json::Value>) -> Result<GetblockstatsResponse, TransportError> {
        self.node_client.getblockstats(hash_or_height, stats).await
    }

/// If the request parameters include a "mode" key, that is used to explicitly select between the default "template" request or a "proposal".
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
/// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
/// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
    pub async fn getblocktemplate(&self, template_request: serde_json::Value) -> Result<(), TransportError> {
        self.node_client.getblocktemplate(template_request).await
    }

/// Return information about chainstates.
    pub async fn getchainstates(&self) -> Result<GetchainstatesResponse, TransportError> {
        self.node_client.getchainstates().await
    }

/// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
    pub async fn getchaintips(&self) -> Result<GetchaintipsResponse, TransportError> {
        self.node_client.getchaintips().await
    }

/// Compute statistics about the total number and rate of transactions in the chain.
    pub async fn getchaintxstats(&self, nblocks: u64, blockhash: bitcoin::BlockHash) -> Result<GetchaintxstatsResponse, TransportError> {
        self.node_client.getchaintxstats(nblocks, blockhash).await
    }

/// Returns the number of connections to other nodes.
    pub async fn getconnectioncount(&self) -> Result<GetconnectioncountResponse, TransportError> {
        self.node_client.getconnectioncount().await
    }

/// Returns an object containing various state info regarding deployments of consensus changes.
    pub async fn getdeploymentinfo(&self, blockhash: bitcoin::BlockHash) -> Result<GetdeploymentinfoResponse, TransportError> {
        self.node_client.getdeploymentinfo(blockhash).await
    }

/// Analyses a descriptor.
    pub async fn getdescriptorinfo(&self, descriptor: String) -> Result<GetdescriptorinfoResponse, TransportError> {
        self.node_client.getdescriptorinfo(descriptor).await
    }

/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
    pub async fn getdifficulty(&self) -> Result<GetdifficultyResponse, TransportError> {
        self.node_client.getdifficulty().await
    }

/// List all BIP 32 HD keys in the wallet and which descriptors use them.
    pub async fn gethdkeys(&self, options: serde_json::Value) -> Result<GethdkeysResponse, TransportError> {
        self.node_client.gethdkeys(options).await
    }

/// Returns the status of one or all available indices currently running in the node.
    pub async fn getindexinfo(&self, index_name: String) -> Result<GetindexinfoResponse, TransportError> {
        self.node_client.getindexinfo(index_name).await
    }

/// Returns an object containing information about memory usage.
    pub async fn getmemoryinfo(&self, mode: String) -> Result<GetmemoryinfoResponse, TransportError> {
        self.node_client.getmemoryinfo(mode).await
    }

/// If txid is in the mempool, returns all in-mempool ancestors.
    pub async fn getmempoolancestors(&self, txid: bitcoin::Txid, verbose: bool) -> Result<GetmempoolancestorsResponse, TransportError> {
        self.node_client.getmempoolancestors(txid, verbose).await
    }

/// If txid is in the mempool, returns all in-mempool descendants.
    pub async fn getmempooldescendants(&self, txid: bitcoin::Txid, verbose: bool) -> Result<GetmempooldescendantsResponse, TransportError> {
        self.node_client.getmempooldescendants(txid, verbose).await
    }

/// Returns mempool data for given transaction
    pub async fn getmempoolentry(&self, txid: bitcoin::Txid) -> Result<GetmempoolentryResponse, TransportError> {
        self.node_client.getmempoolentry(txid).await
    }

/// Returns details on the active state of the TX memory pool.
    pub async fn getmempoolinfo(&self) -> Result<GetmempoolinfoResponse, TransportError> {
        self.node_client.getmempoolinfo().await
    }

/// Returns a json object containing mining-related information.
    pub async fn getmininginfo(&self) -> Result<GetmininginfoResponse, TransportError> {
        self.node_client.getmininginfo().await
    }

/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.
    pub async fn getnettotals(&self) -> Result<GetnettotalsResponse, TransportError> {
        self.node_client.getnettotals().await
    }

/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.
    pub async fn getnetworkhashps(&self, nblocks: u64, height: u64) -> Result<GetnetworkhashpsResponse, TransportError> {
        self.node_client.getnetworkhashps(nblocks, height).await
    }

/// Returns an object containing various state info regarding P2P networking.
    pub async fn getnetworkinfo(&self) -> Result<GetnetworkinfoResponse, TransportError> {
        self.node_client.getnetworkinfo().await
    }

/// Returns a new Bitcoin address for receiving payments.
/// If "label" is specified, it is added to the address book
/// so payments received with the address will be associated with "label".
    pub async fn getnewaddress(&self, label: String, address_type: String) -> Result<GetnewaddressResponse, TransportError> {
        self.wallet_client.getnewaddress(label, address_type).await
    }

/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.
    pub async fn getnodeaddresses(&self, count: u64, network: String) -> Result<GetnodeaddressesResponse, TransportError> {
        self.node_client.getnodeaddresses(count, network).await
    }

/// Shows transactions in the tx orphanage.
///
/// EXPERIMENTAL warning: this call may be changed in future releases.
    pub async fn getorphantxs(&self, verbosity: u32) -> Result<GetorphantxsResponse, TransportError> {
        self.node_client.getorphantxs(verbosity).await
    }

/// Returns data about each connected network peer as a json array of objects.
    pub async fn getpeerinfo(&self) -> Result<GetpeerinfoResponse, TransportError> {
        self.node_client.getpeerinfo().await
    }

/// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
    pub async fn getprioritisedtransactions(&self) -> Result<GetprioritisedtransactionsResponse, TransportError> {
        self.node_client.getprioritisedtransactions().await
    }

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Returns information on all address manager entries for the new and tried tables.
    pub async fn getrawaddrman(&self) -> Result<GetrawaddrmanResponse, TransportError> {
        self.node_client.getrawaddrman().await
    }

/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
    pub async fn getrawchangeaddress(&self, address_type: String) -> Result<GetrawchangeaddressResponse, TransportError> {
        self.wallet_client.getrawchangeaddress(address_type).await
    }

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
///
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
    pub async fn getrawmempool(&self, verbose: bool, mempool_sequence: bool) -> Result<GetrawmempoolResponse, TransportError> {
        self.node_client.getrawmempool(verbose, mempool_sequence).await
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
        self.node_client.getrawtransaction(txid, verbosity, blockhash).await
    }

/// Returns the total amount received by the given address in transactions with at least minconf confirmations.
    pub async fn getreceivedbyaddress(&self, address: String, minconf: u32, include_immature_coinbase: bool) -> Result<GetreceivedbyaddressResponse, TransportError> {
        self.wallet_client.getreceivedbyaddress(address, minconf, include_immature_coinbase).await
    }

/// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
    pub async fn getreceivedbylabel(&self, label: String, minconf: u32, include_immature_coinbase: bool) -> Result<GetreceivedbylabelResponse, TransportError> {
        self.wallet_client.getreceivedbylabel(label, minconf, include_immature_coinbase).await
    }

/// Returns details of the RPC server.
    pub async fn getrpcinfo(&self) -> Result<GetrpcinfoResponse, TransportError> {
        self.node_client.getrpcinfo().await
    }

/// Get detailed information about in-wallet transaction <txid>
    pub async fn gettransaction(&self, txid: bitcoin::Txid, include_watchonly: bool, verbose: bool) -> Result<GettransactionResponse, TransportError> {
        self.wallet_client.gettransaction(txid, include_watchonly, verbose).await
    }

/// Returns details about an unspent transaction output.
    pub async fn gettxout(&self, txid: bitcoin::Txid, n: u32, include_mempool: bool) -> Result<(), TransportError> {
        self.node_client.gettxout(txid, n, include_mempool).await
    }

/// Returns a hex-encoded proof that "txid" was included in a block.
///
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
    pub async fn gettxoutproof(&self, txids: Vec<bitcoin::Txid>, blockhash: bitcoin::BlockHash) -> Result<GettxoutproofResponse, TransportError> {
        self.node_client.gettxoutproof(txids, blockhash).await
    }

/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
    pub async fn gettxoutsetinfo(&self, hash_type: String, hash_or_height: u64, use_index: bool) -> Result<GettxoutsetinfoResponse, TransportError> {
        self.node_client.gettxoutsetinfo(hash_type, hash_or_height, use_index).await
    }

/// Scans the mempool to find transactions spending any of the given outputs
    pub async fn gettxspendingprevout(&self, outputs: Vec<serde_json::Value>) -> Result<GettxspendingprevoutResponse, TransportError> {
        self.node_client.gettxspendingprevout(outputs).await
    }

/// DEPRECATED
/// Identical to getbalances().mine.untrusted_pending
    pub async fn getunconfirmedbalance(&self) -> Result<GetunconfirmedbalanceResponse, TransportError> {
        self.wallet_client.getunconfirmedbalance().await
    }

/// Returns an object containing various wallet state info.
    pub async fn getwalletinfo(&self) -> Result<GetwalletinfoResponse, TransportError> {
        self.wallet_client.getwalletinfo().await
    }

/// List all commands, or get help for a specified command.
    pub async fn help(&self, command: String) -> Result<HelpResponse, TransportError> {
        self.node_client.help(command).await
    }

/// Adds an address or script (in hex) that can be watched as if it were in your wallet but cannot be used to spend. Requires a new wallet backup.
///
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported address exists but related transactions are still missing, leading to temporarily incorrect/bogus balances and unspent outputs until rescan completes.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// If you have the full public key, you should call importpubkey instead of this.
/// Hint: use importmulti to import more than one address.
///
/// Note: If you import a non-standard raw script in hex form, outputs sending to it will be treated
/// as change, and not show up in many RPCs.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" for descriptor wallets.
    pub async fn importaddress(&self, address: String, label: String, rescan: bool, p2sh: bool) -> Result<(), TransportError> {
        self.wallet_client.importaddress(address, label, rescan, p2sh).await
    }

/// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
/// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second elements will be imported as an internal descriptor.
///
/// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
    pub async fn importdescriptors(&self, requests: Vec<serde_json::Value>) -> Result<ImportdescriptorsResponse, TransportError> {
        self.wallet_client.importdescriptors(requests).await
    }

/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
    pub async fn importmempool(&self, filepath: String, options: serde_json::Value) -> Result<ImportmempoolResponse, TransportError> {
        self.node_client.importmempool(filepath, options).await
    }

/// Import addresses/scripts (with private or public keys, redeem script (P2SH)), optionally rescanning the blockchain from the earliest creation time of the imported scripts. Requires a new wallet backup.
/// If an address/script is imported without all of the private keys required to spend from that address, it will be watchonly. The "watchonly" option must be set to true in this case or a warning will be returned.
/// Conversely, if all the private keys are provided and the address/script is spendable, the watchonly option must be set to false, or a warning will be returned.
///
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" for descriptor wallets.
    pub async fn importmulti(&self, requests: Vec<serde_json::Value>, options: serde_json::Value) -> Result<ImportmultiResponse, TransportError> {
        self.wallet_client.importmulti(requests, options).await
    }

/// Adds a private key (as returned by dumpprivkey) to your wallet. Requires a new wallet backup.
/// Hint: use importmulti to import more than one private key.
///
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported key exists but related transactions are still missing, leading to temporarily incorrect/bogus balances and unspent outputs until rescan completes.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" with "combo(X)" for descriptor wallets.
    pub async fn importprivkey(&self, privkey: String, label: String, rescan: bool) -> Result<(), TransportError> {
        self.wallet_client.importprivkey(privkey, label, rescan).await
    }

/// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
    pub async fn importprunedfunds(&self, rawtransaction: String, txoutproof: String) -> Result<(), TransportError> {
        self.wallet_client.importprunedfunds(rawtransaction, txoutproof).await
    }

/// Adds a public key (in hex) that can be watched as if it were in your wallet but cannot be used to spend. Requires a new wallet backup.
/// Hint: use importmulti to import more than one public key.
///
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported pubkey exists but related transactions are still missing, leading to temporarily incorrect/bogus balances and unspent outputs until rescan completes.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" with "combo(X)" for descriptor wallets.
    pub async fn importpubkey(&self, pubkey: String, label: String, rescan: bool) -> Result<(), TransportError> {
        self.wallet_client.importpubkey(pubkey, label, rescan).await
    }

/// Imports keys from a wallet dump file (see dumpwallet). Requires a new wallet backup to include imported keys.
/// Note: Blockchain and Mempool will be rescanned after a successful import. Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets.
    pub async fn importwallet(&self, filename: String) -> Result<(), TransportError> {
        self.wallet_client.importwallet(filename).await
    }

/// Permanently marks a block as invalid, as if it violated a consensus rule.
    pub async fn invalidateblock(&self, blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        self.node_client.invalidateblock(blockhash).await
    }

/// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
/// No input in any of the PSBTs can be in more than one of the PSBTs.
    pub async fn joinpsbts(&self, txs: Vec<serde_json::Value>) -> Result<JoinpsbtsResponse, TransportError> {
        self.node_client.joinpsbts(txs).await
    }

/// Fills the keypool.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn keypoolrefill(&self, newsize: u64) -> Result<(), TransportError> {
        self.wallet_client.keypoolrefill(newsize).await
    }

/// Lists groups of addresses which have had their common ownership
/// made public by common use as inputs or as the resulting change
/// in past transactions
    pub async fn listaddressgroupings(&self) -> Result<ListaddressgroupingsResponse, TransportError> {
        self.wallet_client.listaddressgroupings().await
    }

/// List all manually banned IPs/Subnets.
    pub async fn listbanned(&self) -> Result<ListbannedResponse, TransportError> {
        self.node_client.listbanned().await
    }

/// List descriptors imported into a descriptor-enabled wallet.
    pub async fn listdescriptors(&self, private: bool) -> Result<ListdescriptorsResponse, TransportError> {
        self.wallet_client.listdescriptors(private).await
    }

/// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
    pub async fn listlabels(&self, purpose: String) -> Result<ListlabelsResponse, TransportError> {
        self.wallet_client.listlabels(purpose).await
    }

/// Returns list of temporarily unspendable outputs.
/// See the lockunspent call to lock and unlock transactions for spending.
    pub async fn listlockunspent(&self) -> Result<ListlockunspentResponse, TransportError> {
        self.wallet_client.listlockunspent().await
    }

/// List balances by receiving address.
    pub async fn listreceivedbyaddress(&self, minconf: u32, include_empty: bool, include_watchonly: bool, address_filter: String, include_immature_coinbase: bool) -> Result<ListreceivedbyaddressResponse, TransportError> {
        self.wallet_client.listreceivedbyaddress(minconf, include_empty, include_watchonly, address_filter, include_immature_coinbase).await
    }

/// List received transactions by label.
    pub async fn listreceivedbylabel(&self, minconf: u32, include_empty: bool, include_watchonly: bool, include_immature_coinbase: bool) -> Result<ListreceivedbylabelResponse, TransportError> {
        self.wallet_client.listreceivedbylabel(minconf, include_empty, include_watchonly, include_immature_coinbase).await
    }

/// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
/// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
    pub async fn listsinceblock(&self, blockhash: bitcoin::BlockHash, target_confirmations: u64, include_watchonly: bool, include_removed: bool, include_change: bool, label: String) -> Result<ListsinceblockResponse, TransportError> {
        self.wallet_client.listsinceblock(blockhash, target_confirmations, include_watchonly, include_removed, include_change, label).await
    }

/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
///
/// Returns up to "count" most recent transactions skipping the first "from" transactions.
    pub async fn listtransactions(&self, label: String, count: u64, skip: u64, include_watchonly: bool) -> Result<ListtransactionsResponse, TransportError> {
        self.wallet_client.listtransactions(label, count, skip, include_watchonly).await
    }

/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
    pub async fn listunspent(&self, minconf: u32, maxconf: u32, addresses: Vec<String>, include_unsafe: bool, query_options: serde_json::Value) -> Result<ListunspentResponse, TransportError> {
        self.wallet_client.listunspent(minconf, maxconf, addresses, include_unsafe, query_options).await
    }

/// Returns a list of wallets in the wallet directory.
    pub async fn listwalletdir(&self) -> Result<ListwalletdirResponse, TransportError> {
        self.wallet_client.listwalletdir().await
    }

/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use "getwalletinfo"
    pub async fn listwallets(&self) -> Result<ListwalletsResponse, TransportError> {
        self.wallet_client.listwallets().await
    }

/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network"s tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
///
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
///
/// You can find more information on this process in the ``assumeutxo`` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
    pub async fn loadtxoutset(&self, path: String) -> Result<LoadtxoutsetResponse, TransportError> {
        self.node_client.loadtxoutset(path).await
    }

/// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.
    pub async fn loadwallet(&self, filename: String, load_on_startup: bool) -> Result<LoadwalletResponse, TransportError> {
        self.wallet_client.loadwallet(filename, load_on_startup).await
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
    pub async fn lockunspent(&self, unlock: bool, transactions: Vec<serde_json::Value>, persistent: bool) -> Result<LockunspentResponse, TransportError> {
        self.wallet_client.lockunspent(unlock, transactions, persistent).await
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
        self.node_client.logging(include, exclude).await
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
    pub async fn migratewallet(&self, wallet_name: String, passphrase: String) -> Result<MigratewalletResponse, TransportError> {
        self.wallet_client.migratewallet(wallet_name, passphrase).await
    }

/// Bump the scheduler into the future (-regtest only)
    pub async fn mockscheduler(&self, delta_time: u64) -> Result<(), TransportError> {
        self.node_client.mockscheduler(delta_time).await
    }

/// Entirely clears and refills the keypool.
/// WARNING: On non-HD wallets, this will require a new backup immediately, to include the new keys.
/// When restoring a backup of an HD wallet created before the newkeypool command is run, funds received to
/// new addresses may not appear automatically. They have not been lost, but the wallet may not find them.
/// This can be fixed by running the newkeypool command on the backup and then rescanning, so the wallet
/// re-generates the required keys.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn newkeypool(&self) -> Result<(), TransportError> {
        self.wallet_client.newkeypool().await
    }

/// Requests that a ping be sent to all other nodes, to measure ping time.
/// Results provided in getpeerinfo, pingtime and pingwait fields are decimal seconds.
/// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
    pub async fn ping(&self) -> Result<(), TransportError> {
        self.node_client.ping().await
    }

/// Treats a block as if it were received before others with the same work.
///
/// A later preciousblock call can override the effect of an earlier one.
///
/// The effects of preciousblock are not retained across restarts.
    pub async fn preciousblock(&self, blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        self.node_client.preciousblock(blockhash).await
    }

/// Accepts the transaction into mined blocks at a higher (or lower) priority
    pub async fn prioritisetransaction(&self, txid: bitcoin::Txid, dummy: Option<String>, fee_delta: f64) -> Result<PrioritisetransactionResponse, TransportError> {
        self.node_client.prioritisetransaction(txid, dummy, fee_delta).await
    }


    pub async fn pruneblockchain(&self, height: u64) -> Result<PruneblockchainResponse, TransportError> {
        self.node_client.pruneblockchain(height).await
    }

/// Bumps the fee of an opt-in-RBF transaction T, replacing it with a new transaction B.
/// Returns a PSBT instead of creating and signing a new transaction.
/// An opt-in RBF transaction with the given txid must be in the wallet.
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
    pub async fn psbtbumpfee(&self, txid: bitcoin::Txid, options: serde_json::Value) -> Result<PsbtbumpfeeResponse, TransportError> {
        self.wallet_client.psbtbumpfee(txid, options).await
    }

/// Removes invalidity status of a block, its ancestors and its descendants, reconsider them for activation.
/// This can be used to undo the effects of invalidateblock.
    pub async fn reconsiderblock(&self, blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        self.node_client.reconsiderblock(blockhash).await
    }

/// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
    pub async fn removeprunedfunds(&self, txid: bitcoin::Txid) -> Result<(), TransportError> {
        self.wallet_client.removeprunedfunds(txid).await
    }

/// Rescan the local blockchain for wallet related transactions.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// The rescan is significantly faster when used on a descriptor wallet
/// and block filters are available (using startup option "-blockfilterindex=1").
    pub async fn rescanblockchain(&self, start_height: u64, stop_height: u64) -> Result<RescanblockchainResponse, TransportError> {
        self.wallet_client.rescanblockchain(start_height, stop_height).await
    }

/// Restores and loads a wallet from backup.
///
/// The rescan is significantly faster if a descriptor wallet is restored
/// and block filters are available (using startup option "-blockfilterindex=1").
    pub async fn restorewallet(&self, wallet_name: String, backup_file: String, load_on_startup: bool) -> Result<RestorewalletResponse, TransportError> {
        self.wallet_client.restorewallet(wallet_name, backup_file, load_on_startup).await
    }

/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
    pub async fn savemempool(&self) -> Result<SavemempoolResponse, TransportError> {
        self.node_client.savemempool().await
    }

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn scanblocks(&self, action: String, scanobjects: Vec<serde_json::Value>, start_height: u64, stop_height: u64, filtertype: String, options: serde_json::Value) -> Result<(), TransportError> {
        self.node_client.scanblocks(action, scanobjects, start_height, stop_height, filtertype, options).await
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
        self.node_client.scantxoutset(action, scanobjects).await
    }

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Send a transaction.
    pub async fn send(&self, outputs: Vec<serde_json::Value>, conf_target: u64, estimate_mode: String, fee_rate: f64, options: serde_json::Value) -> Result<SendResponse, TransportError> {
        self.wallet_client.send(outputs, conf_target, estimate_mode, fee_rate, options).await
    }

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
/// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using ``send_max`` to exclude inputs that are worth less than the fees needed to spend them.
    pub async fn sendall(&self, recipients: Vec<serde_json::Value>, conf_target: u64, estimate_mode: String, fee_rate: f64, options: serde_json::Value) -> Result<SendallResponse, TransportError> {
        self.wallet_client.sendall(recipients, conf_target, estimate_mode, fee_rate, options).await
    }

/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sendmany(&self, dummy: Option<String>, amounts: serde_json::Value, minconf: u32, comment: String, subtractfeefrom: Vec<serde_json::Value>, replaceable: bool, conf_target: u64, estimate_mode: String, fee_rate: f64, verbose: bool) -> Result<SendmanyResponse, TransportError> {
        self.wallet_client.sendmany(dummy, amounts, minconf, comment, subtractfeefrom, replaceable, conf_target, estimate_mode, fee_rate, verbose).await
    }

/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.
    pub async fn sendmsgtopeer(&self, peer_id: u64, msg_type: String, msg: String) -> Result<SendmsgtopeerResponse, TransportError> {
        self.node_client.sendmsgtopeer(peer_id, msg_type, msg).await
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
        self.node_client.sendrawtransaction(hexstring, maxfeerate, maxburnamount).await
    }

/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sendtoaddress(&self, address: String, amount: bitcoin::Amount, comment: String, comment_to: String, subtractfeefromamount: bool, replaceable: bool, conf_target: u64, estimate_mode: String, avoid_reuse: bool, fee_rate: f64, verbose: bool) -> Result<SendtoaddressResponse, TransportError> {
        self.wallet_client.sendtoaddress(address, amount, comment, comment_to, subtractfeefromamount, replaceable, conf_target, estimate_mode, avoid_reuse, fee_rate, verbose).await
    }

/// Attempts to add or remove an IP/Subnet from the banned list.
    pub async fn setban(&self, subnet: String, command: String, bantime: u64, absolute: bool) -> Result<(), TransportError> {
        self.node_client.setban(subnet, command, bantime, absolute).await
    }

/// Set or generate a new HD wallet seed. Non-HD wallets will not be upgraded to being a HD wallet. Wallets that are already
/// HD will have a new HD seed set so that new keys added to the keypool will be derived from this new seed.
///
/// Note that you will need to MAKE A NEW BACKUP of your wallet after setting the HD wallet seed.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
/// Note: This command is only compatible with legacy wallets.
    pub async fn sethdseed(&self, newkeypool: bool, seed: String) -> Result<(), TransportError> {
        self.wallet_client.sethdseed(newkeypool, seed).await
    }

/// Sets the label associated with the given address.
    pub async fn setlabel(&self, address: String, label: String) -> Result<(), TransportError> {
        self.wallet_client.setlabel(address, label).await
    }

/// Set the local time to given timestamp (-regtest only)
    pub async fn setmocktime(&self, timestamp: u64) -> Result<(), TransportError> {
        self.node_client.setmocktime(timestamp).await
    }

/// Disable/enable all p2p network activity.
    pub async fn setnetworkactive(&self, state: bool) -> Result<SetnetworkactiveResponse, TransportError> {
        self.node_client.setnetworkactive(state).await
    }

/// Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
/// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
    pub async fn settxfee(&self, amount: bitcoin::Amount) -> Result<SettxfeeResponse, TransportError> {
        self.wallet_client.settxfee(amount).await
    }

/// Change the state of the given wallet flag for a wallet.
    pub async fn setwalletflag(&self, flag: String, value: bool) -> Result<SetwalletflagResponse, TransportError> {
        self.wallet_client.setwalletflag(flag, value).await
    }

/// Sign a message with the private key of an address
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn signmessage(&self, address: String, message: String) -> Result<SignmessageResponse, TransportError> {
        self.wallet_client.signmessage(address, message).await
    }

/// Sign a message with the private key of an address
    pub async fn signmessagewithprivkey(&self, privkey: String, message: String) -> Result<SignmessagewithprivkeyResponse, TransportError> {
        self.node_client.signmessagewithprivkey(privkey, message).await
    }

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
    pub async fn signrawtransactionwithkey(&self, hexstring: String, privkeys: Vec<String>, prevtxs: Vec<serde_json::Value>, sighashtype: String) -> Result<SignrawtransactionwithkeyResponse, TransportError> {
        self.node_client.signrawtransactionwithkey(hexstring, privkeys, prevtxs, sighashtype).await
    }

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn signrawtransactionwithwallet(&self, hexstring: String, prevtxs: Vec<serde_json::Value>, sighashtype: String) -> Result<SignrawtransactionwithwalletResponse, TransportError> {
        self.wallet_client.signrawtransactionwithwallet(hexstring, prevtxs, sighashtype).await
    }

/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
    pub async fn simulaterawtransaction(&self, rawtxs: Vec<serde_json::Value>, options: serde_json::Value) -> Result<SimulaterawtransactionResponse, TransportError> {
        self.wallet_client.simulaterawtransaction(rawtxs, options).await
    }

/// Request a graceful shutdown of Bitcoin Core.
    pub async fn stop(&self, wait: u64) -> Result<StopResponse, TransportError> {
        self.node_client.stop(wait).await
    }

/// Attempts to submit new block to network.
/// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
    pub async fn submitblock(&self, hexdata: String, dummy: Option<String>) -> Result<(), TransportError> {
        self.node_client.submitblock(hexdata, dummy).await
    }

/// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
/// Throws when the header is invalid.
    pub async fn submitheader(&self, hexdata: String) -> Result<(), TransportError> {
        self.node_client.submitheader(hexdata).await
    }

/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
    pub async fn submitpackage(&self, package: Vec<serde_json::Value>, maxfeerate: f64, maxburnamount: f64) -> Result<SubmitpackageResponse, TransportError> {
        self.node_client.submitpackage(package, maxfeerate, maxburnamount).await
    }

/// Waits for the validation interface queue to catch up on everything that was there when we entered this function.
    pub async fn syncwithvalidationinterfacequeue(&self) -> Result<(), TransportError> {
        self.node_client.syncwithvalidationinterfacequeue().await
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
        self.node_client.testmempoolaccept(rawtxs, maxfeerate).await
    }

/// Unloads the wallet referenced by the request endpoint, otherwise unloads the wallet specified in the argument.
/// Specifying the wallet name on a wallet endpoint is invalid.
    pub async fn unloadwallet(&self, wallet_name: String, load_on_startup: bool) -> Result<UnloadwalletResponse, TransportError> {
        self.wallet_client.unloadwallet(wallet_name, load_on_startup).await
    }

/// Upgrade the wallet. Upgrades to the latest version if no version number is specified.
/// New keys may be generated and a new wallet backup will need to be made.
    pub async fn upgradewallet(&self, version: u32) -> Result<UpgradewalletResponse, TransportError> {
        self.wallet_client.upgradewallet(version).await
    }

/// Returns the total uptime of the server.
    pub async fn uptime(&self) -> Result<UptimeResponse, TransportError> {
        self.node_client.uptime().await
    }

/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
    pub async fn utxoupdatepsbt(&self, psbt: String, descriptors: Vec<serde_json::Value>) -> Result<UtxoupdatepsbtResponse, TransportError> {
        self.node_client.utxoupdatepsbt(psbt, descriptors).await
    }

/// Return information about the given bitcoin address.
    pub async fn validateaddress(&self, address: String) -> Result<ValidateaddressResponse, TransportError> {
        self.node_client.validateaddress(address).await
    }

/// Verifies blockchain database.
    pub async fn verifychain(&self, checklevel: u32, nblocks: u64) -> Result<VerifychainResponse, TransportError> {
        self.node_client.verifychain(checklevel, nblocks).await
    }

/// Verify a signed message.
    pub async fn verifymessage(&self, address: String, signature: String, message: String) -> Result<VerifymessageResponse, TransportError> {
        self.node_client.verifymessage(address, signature, message).await
    }

/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
    pub async fn verifytxoutproof(&self, proof: String) -> Result<VerifytxoutproofResponse, TransportError> {
        self.node_client.verifytxoutproof(proof).await
    }

/// Waits for a specific new block and returns useful info about it.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn waitforblock(&self, blockhash: bitcoin::BlockHash, timeout: u64) -> Result<WaitforblockResponse, TransportError> {
        self.node_client.waitforblock(blockhash, timeout).await
    }

/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn waitforblockheight(&self, height: u64, timeout: u64) -> Result<WaitforblockheightResponse, TransportError> {
        self.node_client.waitforblockheight(height, timeout).await
    }

/// Waits for any new block and returns useful info about it.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn waitfornewblock(&self, timeout: u64) -> Result<WaitfornewblockResponse, TransportError> {
        self.node_client.waitfornewblock(timeout).await
    }

/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
    pub async fn walletcreatefundedpsbt(&self, inputs: Vec<serde_json::Value>, outputs: Vec<serde_json::Value>, locktime: u32, options: serde_json::Value, bip32derivs: bool) -> Result<WalletcreatefundedpsbtResponse, TransportError> {
        self.wallet_client.walletcreatefundedpsbt(inputs, outputs, locktime, options, bip32derivs).await
    }

/// Display address on an external signer for verification.
    pub async fn walletdisplayaddress(&self, address: String) -> Result<WalletdisplayaddressResponse, TransportError> {
        self.wallet_client.walletdisplayaddress(address).await
    }

/// Removes the wallet encryption key from memory, locking the wallet.
/// After calling this method, you will need to call walletpassphrase again
/// before being able to call any methods which require the wallet to be unlocked.
    pub async fn walletlock(&self) -> Result<(), TransportError> {
        self.wallet_client.walletlock().await
    }

/// Stores the wallet decryption key in memory for "timeout" seconds.
/// This is needed prior to performing transactions related to private keys such as sending bitcoins
///
/// Note:
/// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
/// time that overrides the old one.
    pub async fn walletpassphrase(&self, passphrase: String, timeout: u64) -> Result<(), TransportError> {
        self.wallet_client.walletpassphrase(passphrase, timeout).await
    }

/// Changes the wallet passphrase from "oldpassphrase" to "newpassphrase".
    pub async fn walletpassphrasechange(&self, oldpassphrase: String, newpassphrase: String) -> Result<(), TransportError> {
        self.wallet_client.walletpassphrasechange(oldpassphrase, newpassphrase).await
    }

/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn walletprocesspsbt(&self, psbt: String, sign: bool, sighashtype: String, bip32derivs: bool, finalize: bool) -> Result<WalletprocesspsbtResponse, TransportError> {
        self.wallet_client.walletprocesspsbt(psbt, sign, sighashtype, bip32derivs, finalize).await
    }

/// Helper method to send bitcoin to an address with either a confirmation target or fee rate.
/// This is a more ergonomic wrapper around sendtoaddress that prevents specifying both conf_target and fee_rate.
/// 
/// Parameters:
/// - address: The destination address
/// - amount: The amount to send
/// - conf_target: The confirmation target in blocks
/// - estimate_mode: The fee estimate mode ("economical" or "conservative")
/// ```

pub async fn send_to_address_with_conf_target(
&self,
address: String,
amount: Amount,
conf_target: u64,
estimate_mode: String,
) -> Result<Value, TransportError> {
Ok(serde_json::to_value(self.wallet_client.sendtoaddress(
address,
amount,
"".to_string(),
"".to_string(),
false,
true,
conf_target,
estimate_mode,
false,
0.0,
false,
).await?)?)
}

pub async fn send_to_address_with_fee_rate(
&self,
address: String,
amount: Amount,
fee_rate: f64,
) -> Result<Value, TransportError> {
Ok(serde_json::to_value(self.wallet_client.sendtoaddress(
address,
amount,
"".to_string(),
"".to_string(),
false,
true,
0u64,
"unset".to_string(),
false,
fee_rate,
false,
).await?)?)
}

}

impl Drop for BitcoinTestClient {
    fn drop(&mut self) {
        let _ = self.node_manager.take();
    }
}

