use std::sync::Arc;

use anyhow::Result;
use bitcoin::{Amount, Network};
use serde_json::Value;

use crate::node::{BitcoinNodeManager, NodeManager as NodeManagerTrait};
use crate::responses::v29_1_responses::*;
use crate::test_config::TestConfig;
use crate::transport::core::{TransportError, TransportExt};
use crate::transport::{BatchBuilder, DefaultTransport, RpcClient};
#[derive(Debug)]
pub struct BitcoinTestClient {
    transport: Arc<DefaultTransport>,
    node_manager: Option<Box<dyn NodeManagerTrait>>,
    /// A thin RPC wrapper around the transport, with batching built in
    rpc: RpcClient,
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
    /// Creates a new Bitcoin test client with default configuration (regtest network).
    /// ```no_run
    /// use bitcoin_rpc_midas::test_node::client::BitcoinTestClient;
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = BitcoinTestClient::new().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new() -> Result<Self, TransportError> {
        tracing::debug!("BitcoinTestClient::new() called");
        let config = TestConfig::default();
        let node_manager = BitcoinNodeManager::new_with_config(&config)?;
        Self::new_with_manager(node_manager).await
    }

    /// Creates a new Bitcoin test client with a specific network.
    /// ```no_run
    /// use bitcoin_rpc_midas::test_node::client::BitcoinTestClient;
    /// use bitcoin::Network;
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = BitcoinTestClient::new_with_network(Network::Bitcoin).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new_with_network(network: Network) -> Result<Self, TransportError> {
        tracing::debug!("BitcoinTestClient::new_with_network({:?}) called", network);
        let config = TestConfig { network, ..Default::default() };
        let node_manager = BitcoinNodeManager::new_with_config(&config)?;
        Self::new_with_manager(node_manager).await
    }

    /// Creates a new Bitcoin test client with a specific node manager.
    /// This allows for custom node configuration and lifecycle management.
    /// The node manager must implement the `NodeManager` trait.
    /// ```no_run
    /// use bitcoin_rpc_midas::test_node::client::BitcoinTestClient;
    /// use bitcoin_rpc_midas::node::BitcoinNodeManager;
    /// use bitcoin_rpc_midas::test_config::TestConfig;
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = TestConfig::default();
    ///     let node_manager = BitcoinNodeManager::new_with_config(&config)?;
    ///     let client = BitcoinTestClient::new_with_manager(node_manager).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new_with_manager<M: NodeManagerTrait + 'static>(
        node_manager: M,
    ) -> Result<Self, TransportError> {
        tracing::debug!("BitcoinTestClient::new_with_manager called");
        // Start the node
        tracing::debug!("Calling node_manager.start()");
        node_manager.start().await?;
        tracing::debug!("node_manager.start() completed successfully");

        // Wait for node to be ready for RPC
        tracing::debug!("Creating transport with port {}", node_manager.rpc_port());
        let transport = Arc::new(DefaultTransport::new(
            format!("http://127.0.0.1:{}", node_manager.rpc_port()),
            Some(("rpcuser".to_string(), "rpcpassword".to_string())),
        ));

        // Create RPC client for batching support
        let rpc = RpcClient::from_transport(transport.clone());

        // Wait for node to be ready for RPC
        // Core initialization states that require waiting:
        // -28: RPC in warmup
        // -4:  RPC in warmup (alternative code)
        let init_states = ["\"code\":-28", "\"code\":-4"];

        let max_retries = 30;
        let mut retries = 0;

        loop {
            match transport.call::<serde_json::Value>("getblockchaininfo", &[]).await {
                Ok(_) => break,
                Err(TransportError::Rpc(e)) => {
                    // Check if the error matches any known initialization state
                    let is_init_state = init_states.iter().any(|state| e.contains(state));
                    if is_init_state && retries < max_retries {
                        tracing::debug!(
                            "Waiting for initialization: {} (attempt {}/{})",
                            e,
                            retries + 1,
                            max_retries
                        );
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
            tracing::debug!("Node initialization completed after {} attempts", retries);
        }

        Ok(Self { transport, node_manager: Some(Box::new(node_manager)), rpc })
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
        let mut params = Vec::new();
        let wallets: ListwalletsResponse = self.transport.call("listwallets", &params).await?;
        if wallets.0.iter().any(|w| w == &wallet_name) {
            params.clear();
            params.push(serde_json::to_value(wallet_name.clone())?);
            params.push(serde_json::to_value(false)?);
            let _: serde_json::Value = self.transport.call("unloadwallet", &params).await?;
        }

        // Try to create wallet
        params.clear();
        params.push(serde_json::to_value(wallet_name.clone())?);
        params.push(serde_json::to_value(opts.disable_private_keys)?);
        params.push(serde_json::to_value(opts.blank)?);
        params.push(serde_json::to_value(opts.passphrase.clone())?);
        params.push(serde_json::to_value(opts.avoid_reuse)?);
        params.push(serde_json::to_value(opts.descriptors)?);
        params.push(serde_json::to_value(opts.load_on_startup)?);
        params.push(serde_json::to_value(opts.external_signer)?);

        match self.transport.call::<CreatewalletResponse>("createwallet", &params).await {
            Ok(_) => Ok(wallet_name),
            Err(TransportError::Rpc(err)) if err.contains("\"code\":-4") => {
                // Try loading instead
                params.clear();
                params.push(serde_json::to_value(wallet_name.clone())?);
                params.push(serde_json::to_value(false)?);
                let _: LoadwalletResponse = self.transport.call("loadwallet", &params).await?;

                // Update transport to use wallet endpoint
                let _new_transport = Arc::new(
                    DefaultTransport::new(
                        format!(
                            "http://127.0.0.1:{}",
                            self.node_manager.as_ref().unwrap().rpc_port()
                        ),
                        Some(("rpcuser".to_string(), "rpcpassword".to_string())),
                    )
                    .with_wallet(wallet_name.clone()),
                );

                // Note: In a real implementation, we'd need to update self.transport here
                // For now, this is a limitation of the current design

                Ok(wallet_name)
            }
            Err(e) => Err(e),
        }
    }

    /// Shortcut for `ensure_wallet_with_options("test_wallet", WalletOptions::default().with_descriptors())`
    pub async fn ensure_default_wallet(
        &mut self,
        name: impl Into<String>,
    ) -> Result<String, TransportError> {
        self.ensure_wallet_with_options(name, WalletOptions::default().with_descriptors()).await
    }

    /// Helper method to mine blocks to a new address
    pub async fn mine_blocks(
        &mut self,
        num_blocks: u64,
        maxtries: u64,
    ) -> Result<(String, Value), TransportError> {
        // Ensure we have a wallet with default settings
        let _wallet_name = self.ensure_default_wallet("test_wallet").await?;

        tracing::debug!("Getting new address");
        let address = self.getnewaddress("".to_string(), "bech32m".to_string()).await?;
        tracing::debug!("Generated address: {:?}", address);
        tracing::debug!("Generating blocks");
        let blocks = self.generatetoaddress(num_blocks, address.0.clone(), maxtries).await?;
        tracing::debug!("Generated blocks: {:?}", blocks);
        Ok((address.0, serde_json::to_value(blocks)?))
    }

    /// Resets the blockchain to a clean state.
    /// This method:
    /// 1. First attempts to prune the blockchain to height 0
    /// 2. If blocks remain, invalidates all blocks except genesis
    /// 3. Reconsiders the genesis block to maintain a valid chain
    pub async fn reset_chain(&mut self) -> Result<(), TransportError> {
        // First try pruning to height 0
        self.pruneblockchain(0).await?;
        // Check if we still have blocks
        let info = self.getblockchaininfo().await?;
        let current_height = info.blocks;
        if current_height > 1 {
            // Invalidate all blocks except genesis
            for height in (1..=current_height).rev() {
                let block_hash = self.getblockhash(height).await?.0;
                self.invalidateblock(block_hash).await?;
            }
            // Reconsider genesis block
            let genesis_hash = self.getblockhash(0).await?.0;
            self.reconsiderblock(genesis_hash).await?;
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
    pub fn node_manager(&self) -> Option<&dyn NodeManagerTrait> { self.node_manager.as_deref() }

    /// Give callers the full RPC client (incl. `.batch()`)
    pub fn rpc(&self) -> &RpcClient { &self.rpc }

    /// Begin a JSON-RPC batch against this test node
    pub fn batch(&self) -> BatchBuilder { self.rpc.batch() }

    /// Mark in-wallet transaction <txid> as abandoned
    /// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
    /// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
    /// It only works on transactions which are not included in a block and are not currently in the mempool.
    /// It has no effect on transactions which are already abandoned.
    pub async fn abandontransaction(&self, txid: bitcoin::Txid) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        self.transport.call("abandontransaction", &params).await
    }

    /// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    pub async fn abortrescan(&self) -> Result<AbortrescanResponse, TransportError> {
        self.transport.call("abortrescan", &[]).await
    }

    /// Open an outbound connection to a specified node. This RPC is for testing only.
    pub async fn addconnection(
        &self,
        address: String,
        connection_type: String,
        v2transport: bool,
    ) -> Result<AddconnectionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(connection_type)?);
        params.push(serde_json::to_value(v2transport)?);
        self.transport.call("addconnection", &params).await
    }

    /// Attempts to add or remove a node from the addnode list.
    /// Or try a connection to a node once.
    /// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
    /// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
    /// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
    pub async fn addnode(
        &self,
        node: String,
        command: String,
        v2transport: bool,
    ) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(node)?);
        params.push(serde_json::to_value(command)?);
        params.push(serde_json::to_value(v2transport)?);
        self.transport.call("addnode", &params).await
    }

    /// Add the address of a potential peer to an address manager table. This RPC is for testing only.
    pub async fn addpeeraddress(
        &self,
        address: String,
        port: u16,
        tried: bool,
    ) -> Result<AddpeeraddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(port)?);
        params.push(serde_json::to_value(tried)?);
        self.transport.call("addpeeraddress", &params).await
    }

    /// Analyzes and provides information about the current status of a PSBT and its inputs
    pub async fn analyzepsbt(&self, psbt: String) -> Result<AnalyzepsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(psbt)?);
        self.transport.call("analyzepsbt", &params).await
    }

    /// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
    pub async fn backupwallet(&self, destination: String) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(destination)?);
        self.transport.call("backupwallet", &params).await
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
    pub async fn bumpfee(
        &self,
        txid: bitcoin::Txid,
        options: serde_json::Value,
    ) -> Result<BumpfeeResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(options)?);
        self.transport.call("bumpfee", &params).await
    }

    /// Clear all banned IPs.
    pub async fn clearbanned(&self) -> Result<(), TransportError> {
        self.transport.call("clearbanned", &[]).await
    }

    /// Combine multiple partially signed Bitcoin transactions into one transaction.
    /// Implements the Combiner role.
    pub async fn combinepsbt(
        &self,
        txs: Vec<serde_json::Value>,
    ) -> Result<CombinepsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txs)?);
        self.transport.call("combinepsbt", &params).await
    }

    /// Combine multiple partially signed transactions into one transaction.
    /// The combined transaction may be another partially signed transaction or a
    /// fully signed transaction.
    pub async fn combinerawtransaction(
        &self,
        txs: Vec<serde_json::Value>,
    ) -> Result<CombinerawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txs)?);
        self.transport.call("combinerawtransaction", &params).await
    }

    /// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
    /// createpsbt and walletcreatefundedpsbt should be used for new applications.
    pub async fn converttopsbt(
        &self,
        hexstring: String,
        permitsigdata: bool,
        iswitness: bool,
    ) -> Result<ConverttopsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        params.push(serde_json::to_value(permitsigdata)?);
        params.push(serde_json::to_value(iswitness)?);
        self.transport.call("converttopsbt", &params).await
    }

    /// Creates a multi-signature address with n signatures of m keys required.
    /// It returns a json object with the address and redeemScript.
    pub async fn createmultisig(
        &self,
        nrequired: u32,
        keys: Vec<String>,
        address_type: String,
    ) -> Result<CreatemultisigResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(nrequired)?);
        params.push(serde_json::to_value(keys)?);
        params.push(serde_json::to_value(address_type)?);
        self.transport.call("createmultisig", &params).await
    }

    /// Creates a transaction in the Partially Signed Transaction format.
    /// Implements the Creator role.
    /// Note that the transaction"s inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    pub async fn createpsbt(
        &self,
        inputs: Vec<serde_json::Value>,
        outputs: Vec<serde_json::Value>,
        locktime: u32,
        replaceable: bool,
        version: u32,
    ) -> Result<CreatepsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(inputs)?);
        params.push(serde_json::to_value(outputs)?);
        params.push(serde_json::to_value(locktime)?);
        params.push(serde_json::to_value(replaceable)?);
        params.push(serde_json::to_value(version)?);
        self.transport.call("createpsbt", &params).await
    }

    /// Create a transaction spending the given inputs and creating new outputs.
    /// Outputs can be addresses or data.
    /// Returns hex-encoded raw transaction.
    /// Note that the transaction"s inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    pub async fn createrawtransaction(
        &self,
        inputs: Vec<serde_json::Value>,
        outputs: Vec<serde_json::Value>,
        locktime: u32,
        replaceable: bool,
        version: u32,
    ) -> Result<CreaterawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(inputs)?);
        params.push(serde_json::to_value(outputs)?);
        params.push(serde_json::to_value(locktime)?);
        params.push(serde_json::to_value(replaceable)?);
        params.push(serde_json::to_value(version)?);
        self.transport.call("createrawtransaction", &params).await
    }

    /// Creates and loads a new wallet.
    pub async fn createwallet(
        &self,
        wallet_name: String,
        disable_private_keys: bool,
        blank: bool,
        passphrase: String,
        avoid_reuse: bool,
        descriptors: bool,
        load_on_startup: bool,
        external_signer: bool,
    ) -> Result<CreatewalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(wallet_name)?);
        params.push(serde_json::to_value(disable_private_keys)?);
        params.push(serde_json::to_value(blank)?);
        params.push(serde_json::to_value(passphrase)?);
        params.push(serde_json::to_value(avoid_reuse)?);
        params.push(serde_json::to_value(descriptors)?);
        params.push(serde_json::to_value(load_on_startup)?);
        params.push(serde_json::to_value(external_signer)?);
        self.transport.call("createwallet", &params).await
    }

    /// Creates the wallet"s descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn createwalletdescriptor(
        &self,
        _type: String,
        options: serde_json::Value,
    ) -> Result<CreatewalletdescriptorResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(_type)?);
        params.push(serde_json::to_value(options)?);
        self.transport.call("createwalletdescriptor", &params).await
    }

    /// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
    pub async fn decodepsbt(&self, psbt: String) -> Result<DecodepsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(psbt)?);
        self.transport.call("decodepsbt", &params).await
    }

    /// Return a JSON object representing the serialized, hex-encoded transaction.
    pub async fn decoderawtransaction(
        &self,
        hexstring: String,
        iswitness: bool,
    ) -> Result<DecoderawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        params.push(serde_json::to_value(iswitness)?);
        self.transport.call("decoderawtransaction", &params).await
    }

    /// Decode a hex-encoded script.
    pub async fn decodescript(
        &self,
        hexstring: String,
    ) -> Result<DecodescriptResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        self.transport.call("decodescript", &params).await
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
    pub async fn deriveaddresses(
        &self,
        descriptor: String,
        range: serde_json::Value,
    ) -> Result<DeriveaddressesResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(descriptor)?);
        params.push(serde_json::to_value(range)?);
        self.transport.call("deriveaddresses", &params).await
    }

    /// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
    /// Then, sign the inputs we are able to with information from the output descriptors.
    pub async fn descriptorprocesspsbt(
        &self,
        psbt: String,
        descriptors: Vec<serde_json::Value>,
        sighashtype: String,
        bip32derivs: bool,
        finalize: bool,
    ) -> Result<DescriptorprocesspsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(psbt)?);
        params.push(serde_json::to_value(descriptors)?);
        params.push(serde_json::to_value(sighashtype)?);
        params.push(serde_json::to_value(bip32derivs)?);
        params.push(serde_json::to_value(finalize)?);
        self.transport.call("descriptorprocesspsbt", &params).await
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
        self.transport.call("disconnectnode", &params).await
    }

    /// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
    ///
    /// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
    ///
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn dumptxoutset(
        &self,
        path: String,
        _type: String,
        options: serde_json::Value,
    ) -> Result<DumptxoutsetResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(path)?);
        params.push(serde_json::to_value(_type)?);
        params.push(serde_json::to_value(options)?);
        self.transport.call("dumptxoutset", &params).await
    }

    /// Simply echo back the input arguments. This command is for testing.
    ///
    /// It will return an internal bug report when arg9="trigger_internal_bug" is passed.
    ///
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    pub async fn echo(
        &self,
        arg0: String,
        arg1: String,
        arg2: String,
        arg3: String,
        arg4: String,
        arg5: String,
        arg6: String,
        arg7: String,
        arg8: String,
        arg9: String,
    ) -> Result<EchoResponse, TransportError> {
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
        self.transport.call("echo", &params).await
    }

    /// Echo back the input argument, passing it through a spawned process in a multiprocess build.
    /// This command is for testing.
    pub async fn echoipc(&self, arg: String) -> Result<EchoipcResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(arg)?);
        self.transport.call("echoipc", &params).await
    }

    /// Simply echo back the input arguments. This command is for testing.
    ///
    /// It will return an internal bug report when arg9="trigger_internal_bug" is passed.
    ///
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    pub async fn echojson(
        &self,
        arg0: String,
        arg1: String,
        arg2: String,
        arg3: String,
        arg4: String,
        arg5: String,
        arg6: String,
        arg7: String,
        arg8: String,
        arg9: String,
    ) -> Result<EchojsonResponse, TransportError> {
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
        self.transport.call("echojson", &params).await
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
    pub async fn encryptwallet(
        &self,
        passphrase: String,
    ) -> Result<EncryptwalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(passphrase)?);
        self.transport.call("encryptwallet", &params).await
    }

    /// Returns a list of external signers from -signer.
    pub async fn enumeratesigners(&self) -> Result<EnumeratesignersResponse, TransportError> {
        self.transport.call("enumeratesigners", &[]).await
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
    pub async fn estimaterawfee(
        &self,
        conf_target: u64,
        threshold: u64,
    ) -> Result<EstimaterawfeeResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(conf_target)?);
        params.push(serde_json::to_value(threshold)?);
        self.transport.call("estimaterawfee", &params).await
    }

    /// Estimates the approximate fee per kilobyte needed for a transaction to begin
    /// confirmation within conf_target blocks if possible and return the number of blocks
    /// for which the estimate is valid. Uses virtual transaction size as defined
    /// in BIP 141 (witness data is discounted).
    pub async fn estimatesmartfee(
        &self,
        conf_target: u64,
        estimate_mode: String,
    ) -> Result<EstimatesmartfeeResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(conf_target)?);
        params.push(serde_json::to_value(estimate_mode)?);
        self.transport.call("estimatesmartfee", &params).await
    }

    /// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
    /// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
    /// created which has the final_scriptSig and final_scriptWitness fields filled for inputs that are complete.
    /// Implements the Finalizer and Extractor roles.
    pub async fn finalizepsbt(
        &self,
        psbt: String,
        extract: bool,
    ) -> Result<FinalizepsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(psbt)?);
        params.push(serde_json::to_value(extract)?);
        self.transport.call("finalizepsbt", &params).await
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
    pub async fn fundrawtransaction(
        &self,
        hexstring: String,
        options: serde_json::Value,
        iswitness: bool,
    ) -> Result<FundrawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        params.push(serde_json::to_value(options)?);
        params.push(serde_json::to_value(iswitness)?);
        self.transport.call("fundrawtransaction", &params).await
    }

    /// has been replaced by the -generate cli option. Refer to -help for more information.
    pub async fn generate(&self) -> Result<(), TransportError> {
        self.transport.call("generate", &[]).await
    }

    /// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
    pub async fn generateblock(
        &self,
        output: String,
        transactions: Vec<serde_json::Value>,
        submit: bool,
    ) -> Result<GenerateblockResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(output)?);
        params.push(serde_json::to_value(transactions)?);
        params.push(serde_json::to_value(submit)?);
        self.transport.call("generateblock", &params).await
    }

    /// Mine to a specified address and return the block hashes.
    pub async fn generatetoaddress(
        &self,
        nblocks: u64,
        address: String,
        maxtries: u64,
    ) -> Result<GeneratetoaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(nblocks)?);
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(maxtries)?);
        self.transport.call("generatetoaddress", &params).await
    }

    /// Mine to a specified descriptor and return the block hashes.
    pub async fn generatetodescriptor(
        &self,
        num_blocks: u64,
        descriptor: String,
        maxtries: u64,
    ) -> Result<GeneratetodescriptorResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(num_blocks)?);
        params.push(serde_json::to_value(descriptor)?);
        params.push(serde_json::to_value(maxtries)?);
        self.transport.call("generatetodescriptor", &params).await
    }

    /// Returns information about the given added node, or all added nodes
    /// (note that onetry addnodes are not listed here)
    pub async fn getaddednodeinfo(
        &self,
        node: String,
    ) -> Result<GetaddednodeinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(node)?);
        self.transport.call("getaddednodeinfo", &params).await
    }

    /// Returns the list of addresses assigned the specified label.
    pub async fn getaddressesbylabel(
        &self,
        label: String,
    ) -> Result<GetaddressesbylabelResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(label)?);
        self.transport.call("getaddressesbylabel", &params).await
    }

    /// Return information about the given bitcoin address.
    /// Some of the information will only be present if the address is in the active wallet.
    pub async fn getaddressinfo(
        &self,
        address: String,
    ) -> Result<GetaddressinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        self.transport.call("getaddressinfo", &params).await
    }

    /// Provides information about the node"s address manager by returning the number of addresses in the ``new`` and ``tried`` tables and their sum for all networks.
    pub async fn getaddrmaninfo(&self) -> Result<GetaddrmaninfoResponse, TransportError> {
        self.transport.call("getaddrmaninfo", &[]).await
    }

    /// Returns the total available balance.
    /// The available balance is what the wallet considers currently spendable, and is
    /// thus affected by options which limit spendability such as -spendzeroconfchange.
    pub async fn getbalance(
        &self,
        dummy: Option<String>,
        minconf: u32,
        include_watchonly: bool,
        avoid_reuse: bool,
    ) -> Result<GetbalanceResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(dummy)?);
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(include_watchonly)?);
        params.push(serde_json::to_value(avoid_reuse)?);
        self.transport.call("getbalance", &params).await
    }

    /// Returns an object with all balances in BTC.
    pub async fn getbalances(&self) -> Result<GetbalancesResponse, TransportError> {
        self.transport.call("getbalances", &[]).await
    }

    /// Returns the hash of the best (tip) block in the most-work fully-validated chain.
    pub async fn getbestblockhash(&self) -> Result<GetbestblockhashResponse, TransportError> {
        self.transport.call("getbestblockhash", &[]).await
    }

    /// If verbosity is 0, returns a string that is serialized, hex-encoded data for block "hash".
    /// If verbosity is 1, returns an Object with information about block <hash>.
    /// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
    /// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
    pub async fn getblock(
        &self,
        blockhash: bitcoin::BlockHash,
        verbosity: u32,
    ) -> Result<GetblockResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        params.push(serde_json::to_value(verbosity)?);
        self.transport.call("getblock", &params).await
    }

    /// Returns an object containing various state info regarding blockchain processing.
    pub async fn getblockchaininfo(&self) -> Result<GetblockchaininfoResponse, TransportError> {
        self.transport.call("getblockchaininfo", &[]).await
    }

    /// Returns the height of the most-work fully-validated chain.
    /// The genesis block has height 0.
    pub async fn getblockcount(&self) -> Result<GetblockcountResponse, TransportError> {
        self.transport.call("getblockcount", &[]).await
    }

    /// Retrieve a BIP 157 content filter for a particular block.
    pub async fn getblockfilter(
        &self,
        blockhash: bitcoin::BlockHash,
        filtertype: String,
    ) -> Result<GetblockfilterResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        params.push(serde_json::to_value(filtertype)?);
        self.transport.call("getblockfilter", &params).await
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
    pub async fn getblockfrompeer(
        &self,
        blockhash: bitcoin::BlockHash,
        peer_id: u64,
    ) -> Result<GetblockfrompeerResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        params.push(serde_json::to_value(peer_id)?);
        self.transport.call("getblockfrompeer", &params).await
    }

    /// Returns hash of block in best-block-chain at height provided.
    pub async fn getblockhash(&self, height: u64) -> Result<GetblockhashResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(height)?);
        self.transport.call("getblockhash", &params).await
    }

    /// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader "hash".
    /// If verbose is true, returns an Object with information about blockheader <hash>.
    pub async fn getblockheader(
        &self,
        blockhash: bitcoin::BlockHash,
        verbose: bool,
    ) -> Result<GetblockheaderResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        params.push(serde_json::to_value(verbose)?);
        self.transport.call("getblockheader", &params).await
    }

    /// Compute per block statistics for a given window. All amounts are in satoshis.
    /// It won"t work for some heights with pruning.
    pub async fn getblockstats(
        &self,
        hash_or_height: serde_json::Value,
        stats: Vec<serde_json::Value>,
    ) -> Result<GetblockstatsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hash_or_height)?);
        params.push(serde_json::to_value(stats)?);
        self.transport.call("getblockstats", &params).await
    }

    /// If the request parameters include a "mode" key, that is used to explicitly select between the default "template" request or a "proposal".
    /// It returns data needed to construct a block to work on.
    /// For full specification, see BIPs 22, 23, 9, and 145:
    /// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
    /// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
    /// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
    /// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
    pub async fn getblocktemplate(
        &self,
        template_request: serde_json::Value,
    ) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(template_request)?);
        self.transport.call("getblocktemplate", &params).await
    }

    /// Return information about chainstates.
    pub async fn getchainstates(&self) -> Result<GetchainstatesResponse, TransportError> {
        self.transport.call("getchainstates", &[]).await
    }

    /// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
    pub async fn getchaintips(&self) -> Result<GetchaintipsResponse, TransportError> {
        self.transport.call("getchaintips", &[]).await
    }

    /// Compute statistics about the total number and rate of transactions in the chain.
    pub async fn getchaintxstats(
        &self,
        nblocks: u64,
        blockhash: bitcoin::BlockHash,
    ) -> Result<GetchaintxstatsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(nblocks)?);
        params.push(serde_json::to_value(blockhash)?);
        self.transport.call("getchaintxstats", &params).await
    }

    /// Returns the number of connections to other nodes.
    pub async fn getconnectioncount(&self) -> Result<GetconnectioncountResponse, TransportError> {
        self.transport.call("getconnectioncount", &[]).await
    }

    /// Returns an object containing various state info regarding deployments of consensus changes.
    pub async fn getdeploymentinfo(
        &self,
        blockhash: bitcoin::BlockHash,
    ) -> Result<GetdeploymentinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        self.transport.call("getdeploymentinfo", &params).await
    }

    /// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the ``relevant_blocks`` output of ``scanblocks()``.
    /// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn getdescriptoractivity(
        &self,
        blockhashes: Vec<serde_json::Value>,
        scanobjects: Vec<serde_json::Value>,
        include_mempool: bool,
    ) -> Result<GetdescriptoractivityResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhashes)?);
        params.push(serde_json::to_value(scanobjects)?);
        params.push(serde_json::to_value(include_mempool)?);
        self.transport.call("getdescriptoractivity", &params).await
    }

    /// Analyses a descriptor.
    pub async fn getdescriptorinfo(
        &self,
        descriptor: String,
    ) -> Result<GetdescriptorinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(descriptor)?);
        self.transport.call("getdescriptorinfo", &params).await
    }

    /// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
    pub async fn getdifficulty(&self) -> Result<GetdifficultyResponse, TransportError> {
        self.transport.call("getdifficulty", &[]).await
    }

    /// List all BIP 32 HD keys in the wallet and which descriptors use them.
    pub async fn gethdkeys(
        &self,
        options: serde_json::Value,
    ) -> Result<GethdkeysResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(options)?);
        self.transport.call("gethdkeys", &params).await
    }

    /// Returns the status of one or all available indices currently running in the node.
    pub async fn getindexinfo(
        &self,
        index_name: String,
    ) -> Result<GetindexinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(index_name)?);
        self.transport.call("getindexinfo", &params).await
    }

    /// Returns an object containing information about memory usage.
    pub async fn getmemoryinfo(
        &self,
        mode: String,
    ) -> Result<GetmemoryinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(mode)?);
        self.transport.call("getmemoryinfo", &params).await
    }

    /// If txid is in the mempool, returns all in-mempool ancestors.
    pub async fn getmempoolancestors(
        &self,
        txid: bitcoin::Txid,
        verbose: bool,
    ) -> Result<GetmempoolancestorsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(verbose)?);
        self.transport.call("getmempoolancestors", &params).await
    }

    /// If txid is in the mempool, returns all in-mempool descendants.
    pub async fn getmempooldescendants(
        &self,
        txid: bitcoin::Txid,
        verbose: bool,
    ) -> Result<GetmempooldescendantsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(verbose)?);
        self.transport.call("getmempooldescendants", &params).await
    }

    /// Returns mempool data for given transaction
    pub async fn getmempoolentry(
        &self,
        txid: bitcoin::Txid,
    ) -> Result<GetmempoolentryResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        self.transport.call("getmempoolentry", &params).await
    }

    /// Returns details on the active state of the TX memory pool.
    pub async fn getmempoolinfo(&self) -> Result<GetmempoolinfoResponse, TransportError> {
        self.transport.call("getmempoolinfo", &[]).await
    }

    /// Returns a json object containing mining-related information.
    pub async fn getmininginfo(&self) -> Result<GetmininginfoResponse, TransportError> {
        self.transport.call("getmininginfo", &[]).await
    }

    /// Returns information about network traffic, including bytes in, bytes out,
    /// and current system time.
    pub async fn getnettotals(&self) -> Result<GetnettotalsResponse, TransportError> {
        self.transport.call("getnettotals", &[]).await
    }

    /// Returns the estimated network hashes per second based on the last n blocks.
    /// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
    /// Pass in [height] to estimate the network speed at the time when a certain block was found.
    pub async fn getnetworkhashps(
        &self,
        nblocks: u64,
        height: u64,
    ) -> Result<GetnetworkhashpsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(nblocks)?);
        params.push(serde_json::to_value(height)?);
        self.transport.call("getnetworkhashps", &params).await
    }

    /// Returns an object containing various state info regarding P2P networking.
    pub async fn getnetworkinfo(&self) -> Result<GetnetworkinfoResponse, TransportError> {
        self.transport.call("getnetworkinfo", &[]).await
    }

    /// Returns a new Bitcoin address for receiving payments.
    /// If "label" is specified, it is added to the address book
    /// so payments received with the address will be associated with "label".
    pub async fn getnewaddress(
        &self,
        label: String,
        address_type: String,
    ) -> Result<GetnewaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(label)?);
        params.push(serde_json::to_value(address_type)?);
        self.transport.call("getnewaddress", &params).await
    }

    /// Return known addresses, after filtering for quality and recency.
    /// These can potentially be used to find new peers in the network.
    /// The total number of addresses known to the node may be higher.
    pub async fn getnodeaddresses(
        &self,
        count: u64,
        network: String,
    ) -> Result<GetnodeaddressesResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(count)?);
        params.push(serde_json::to_value(network)?);
        self.transport.call("getnodeaddresses", &params).await
    }

    /// Shows transactions in the tx orphanage.
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    pub async fn getorphantxs(
        &self,
        verbosity: u32,
    ) -> Result<GetorphantxsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(verbosity)?);
        self.transport.call("getorphantxs", &params).await
    }

    /// Returns data about each connected network peer as a json array of objects.
    pub async fn getpeerinfo(&self) -> Result<GetpeerinfoResponse, TransportError> {
        self.transport.call("getpeerinfo", &[]).await
    }

    /// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
    pub async fn getprioritisedtransactions(
        &self,
    ) -> Result<GetprioritisedtransactionsResponse, TransportError> {
        self.transport.call("getprioritisedtransactions", &[]).await
    }

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Returns information on all address manager entries for the new and tried tables.
    pub async fn getrawaddrman(&self) -> Result<GetrawaddrmanResponse, TransportError> {
        self.transport.call("getrawaddrman", &[]).await
    }

    /// Returns a new Bitcoin address, for receiving change.
    /// This is for use with raw transactions, NOT normal use.
    pub async fn getrawchangeaddress(
        &self,
        address_type: String,
    ) -> Result<GetrawchangeaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address_type)?);
        self.transport.call("getrawchangeaddress", &params).await
    }

    /// Returns all transaction ids in memory pool as a json array of string transaction ids.
    ///
    /// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
    pub async fn getrawmempool(
        &self,
        verbose: bool,
        mempool_sequence: bool,
    ) -> Result<GetrawmempoolResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(verbose)?);
        params.push(serde_json::to_value(mempool_sequence)?);
        self.transport.call("getrawmempool", &params).await
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
    pub async fn getrawtransaction(
        &self,
        txid: bitcoin::Txid,
        verbosity: u32,
        blockhash: bitcoin::BlockHash,
    ) -> Result<GetrawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(verbosity)?);
        params.push(serde_json::to_value(blockhash)?);
        self.transport.call("getrawtransaction", &params).await
    }

    /// Returns the total amount received by the given address in transactions with at least minconf confirmations.
    pub async fn getreceivedbyaddress(
        &self,
        address: String,
        minconf: u32,
        include_immature_coinbase: bool,
    ) -> Result<GetreceivedbyaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(include_immature_coinbase)?);
        self.transport.call("getreceivedbyaddress", &params).await
    }

    /// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
    pub async fn getreceivedbylabel(
        &self,
        label: String,
        minconf: u32,
        include_immature_coinbase: bool,
    ) -> Result<GetreceivedbylabelResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(label)?);
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(include_immature_coinbase)?);
        self.transport.call("getreceivedbylabel", &params).await
    }

    /// Returns details of the RPC server.
    pub async fn getrpcinfo(&self) -> Result<GetrpcinfoResponse, TransportError> {
        self.transport.call("getrpcinfo", &[]).await
    }

    /// Get detailed information about in-wallet transaction <txid>
    pub async fn gettransaction(
        &self,
        txid: bitcoin::Txid,
        include_watchonly: bool,
        verbose: bool,
    ) -> Result<GettransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(include_watchonly)?);
        params.push(serde_json::to_value(verbose)?);
        self.transport.call("gettransaction", &params).await
    }

    /// Returns details about an unspent transaction output.
    pub async fn gettxout(
        &self,
        txid: bitcoin::Txid,
        n: u32,
        include_mempool: bool,
    ) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(n)?);
        params.push(serde_json::to_value(include_mempool)?);
        self.transport.call("gettxout", &params).await
    }

    /// Returns a hex-encoded proof that "txid" was included in a block.
    ///
    /// NOTE: By default this function only works sometimes. This is when there is an
    /// unspent output in the utxo for this transaction. To make it always work,
    /// you need to maintain a transaction index, using the -txindex command line option or
    /// specify the block in which the transaction is included manually (by blockhash).
    pub async fn gettxoutproof(
        &self,
        txids: Vec<bitcoin::Txid>,
        blockhash: bitcoin::BlockHash,
    ) -> Result<GettxoutproofResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txids)?);
        params.push(serde_json::to_value(blockhash)?);
        self.transport.call("gettxoutproof", &params).await
    }

    /// Returns statistics about the unspent transaction output set.
    /// Note this call may take some time if you are not using coinstatsindex.
    pub async fn gettxoutsetinfo(
        &self,
        hash_type: String,
        hash_or_height: serde_json::Value,
        use_index: bool,
    ) -> Result<GettxoutsetinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hash_type)?);
        params.push(serde_json::to_value(hash_or_height)?);
        params.push(serde_json::to_value(use_index)?);
        self.transport.call("gettxoutsetinfo", &params).await
    }

    /// Scans the mempool to find transactions spending any of the given outputs
    pub async fn gettxspendingprevout(
        &self,
        outputs: Vec<serde_json::Value>,
    ) -> Result<GettxspendingprevoutResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(outputs)?);
        self.transport.call("gettxspendingprevout", &params).await
    }

    /// Returns an object containing various wallet state info.
    pub async fn getwalletinfo(&self) -> Result<GetwalletinfoResponse, TransportError> {
        self.transport.call("getwalletinfo", &[]).await
    }

    /// Returns information about the active ZeroMQ notifications.
    pub async fn getzmqnotifications(&self) -> Result<GetzmqnotificationsResponse, TransportError> {
        self.transport.call("getzmqnotifications", &[]).await
    }

    /// List all commands, or get help for a specified command.
    pub async fn help(&self, command: String) -> Result<HelpResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(command)?);
        self.transport.call("help", &params).await
    }

    /// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
    /// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second element will be imported as an internal descriptor.
    ///
    /// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
    /// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
    /// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
    pub async fn importdescriptors(
        &self,
        requests: Vec<serde_json::Value>,
    ) -> Result<ImportdescriptorsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(requests)?);
        self.transport.call("importdescriptors", &params).await
    }

    /// Import a mempool.dat file and attempt to add its contents to the mempool.
    /// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
    pub async fn importmempool(
        &self,
        filepath: String,
        options: serde_json::Value,
    ) -> Result<ImportmempoolResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(filepath)?);
        params.push(serde_json::to_value(options)?);
        self.transport.call("importmempool", &params).await
    }

    /// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
    pub async fn importprunedfunds(
        &self,
        rawtransaction: String,
        txoutproof: String,
    ) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(rawtransaction)?);
        params.push(serde_json::to_value(txoutproof)?);
        self.transport.call("importprunedfunds", &params).await
    }

    /// Permanently marks a block as invalid, as if it violated a consensus rule.
    pub async fn invalidateblock(
        &self,
        blockhash: bitcoin::BlockHash,
    ) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        self.transport.call("invalidateblock", &params).await
    }

    /// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
    /// No input in any of the PSBTs can be in more than one of the PSBTs.
    pub async fn joinpsbts(
        &self,
        txs: Vec<serde_json::Value>,
    ) -> Result<JoinpsbtsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txs)?);
        self.transport.call("joinpsbts", &params).await
    }

    /// Refills each descriptor keypool in the wallet up to the specified number of new keys.
    /// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
    ///
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn keypoolrefill(&self, newsize: u64) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(newsize)?);
        self.transport.call("keypoolrefill", &params).await
    }

    /// Lists groups of addresses which have had their common ownership
    /// made public by common use as inputs or as the resulting change
    /// in past transactions
    pub async fn listaddressgroupings(
        &self,
    ) -> Result<ListaddressgroupingsResponse, TransportError> {
        self.transport.call("listaddressgroupings", &[]).await
    }

    /// List all manually banned IPs/Subnets.
    pub async fn listbanned(&self) -> Result<ListbannedResponse, TransportError> {
        self.transport.call("listbanned", &[]).await
    }

    /// List all descriptors present in a wallet.
    pub async fn listdescriptors(
        &self,
        private: bool,
    ) -> Result<ListdescriptorsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(private)?);
        self.transport.call("listdescriptors", &params).await
    }

    /// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
    pub async fn listlabels(&self, purpose: String) -> Result<ListlabelsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(purpose)?);
        self.transport.call("listlabels", &params).await
    }

    /// Returns list of temporarily unspendable outputs.
    /// See the lockunspent call to lock and unlock transactions for spending.
    pub async fn listlockunspent(&self) -> Result<ListlockunspentResponse, TransportError> {
        self.transport.call("listlockunspent", &[]).await
    }

    /// List balances by receiving address.
    pub async fn listreceivedbyaddress(
        &self,
        minconf: u32,
        include_empty: bool,
        include_watchonly: bool,
        address_filter: String,
        include_immature_coinbase: bool,
    ) -> Result<ListreceivedbyaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(include_empty)?);
        params.push(serde_json::to_value(include_watchonly)?);
        params.push(serde_json::to_value(address_filter)?);
        params.push(serde_json::to_value(include_immature_coinbase)?);
        self.transport.call("listreceivedbyaddress", &params).await
    }

    /// List received transactions by label.
    pub async fn listreceivedbylabel(
        &self,
        minconf: u32,
        include_empty: bool,
        include_watchonly: bool,
        include_immature_coinbase: bool,
    ) -> Result<ListreceivedbylabelResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(include_empty)?);
        params.push(serde_json::to_value(include_watchonly)?);
        params.push(serde_json::to_value(include_immature_coinbase)?);
        self.transport.call("listreceivedbylabel", &params).await
    }

    /// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
    /// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
    /// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
    pub async fn listsinceblock(
        &self,
        blockhash: bitcoin::BlockHash,
        target_confirmations: u64,
        include_watchonly: bool,
        include_removed: bool,
        include_change: bool,
        label: String,
    ) -> Result<ListsinceblockResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        params.push(serde_json::to_value(target_confirmations)?);
        params.push(serde_json::to_value(include_watchonly)?);
        params.push(serde_json::to_value(include_removed)?);
        params.push(serde_json::to_value(include_change)?);
        params.push(serde_json::to_value(label)?);
        self.transport.call("listsinceblock", &params).await
    }

    /// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
    ///
    /// Returns up to "count" most recent transactions skipping the first "from" transactions.
    pub async fn listtransactions(
        &self,
        label: String,
        count: u64,
        skip: u64,
        include_watchonly: bool,
    ) -> Result<ListtransactionsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(label)?);
        params.push(serde_json::to_value(count)?);
        params.push(serde_json::to_value(skip)?);
        params.push(serde_json::to_value(include_watchonly)?);
        self.transport.call("listtransactions", &params).await
    }

    /// Returns array of unspent transaction outputs
    /// with between minconf and maxconf (inclusive) confirmations.
    /// Optionally filter to only include txouts paid to specified addresses.
    pub async fn listunspent(
        &self,
        minconf: u32,
        maxconf: u32,
        addresses: Vec<String>,
        include_unsafe: bool,
        query_options: serde_json::Value,
    ) -> Result<ListunspentResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(maxconf)?);
        params.push(serde_json::to_value(addresses)?);
        params.push(serde_json::to_value(include_unsafe)?);
        params.push(serde_json::to_value(query_options)?);
        self.transport.call("listunspent", &params).await
    }

    /// Returns a list of wallets in the wallet directory.
    pub async fn listwalletdir(&self) -> Result<ListwalletdirResponse, TransportError> {
        self.transport.call("listwalletdir", &[]).await
    }

    /// Returns a list of currently loaded wallets.
    /// For full information on the wallet, use "getwalletinfo"
    pub async fn listwallets(&self) -> Result<ListwalletsResponse, TransportError> {
        self.transport.call("listwallets", &[]).await
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
        self.transport.call("loadtxoutset", &params).await
    }

    /// Loads a wallet from a wallet file or directory.
    /// Note that all wallet command-line options used when starting bitcoind will be
    /// applied to the new wallet.
    pub async fn loadwallet(
        &self,
        filename: String,
        load_on_startup: bool,
    ) -> Result<LoadwalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(filename)?);
        params.push(serde_json::to_value(load_on_startup)?);
        self.transport.call("loadwallet", &params).await
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
    pub async fn lockunspent(
        &self,
        unlock: bool,
        transactions: Vec<serde_json::Value>,
        persistent: bool,
    ) -> Result<LockunspentResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(unlock)?);
        params.push(serde_json::to_value(transactions)?);
        params.push(serde_json::to_value(persistent)?);
        self.transport.call("lockunspent", &params).await
    }

    /// Gets and sets the logging configuration.
    /// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
    /// When called with arguments, adds or removes categories from debug logging and return the lists above.
    /// The arguments are evaluated in order "include", "exclude".
    /// If an item is both included and excluded, it will thus end up being excluded.
    /// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
    /// In addition, the following are available as category names with special meanings:
    /// - "all",  "1" : represent all logging categories.
    pub async fn logging(
        &self,
        include: Vec<serde_json::Value>,
        exclude: Vec<serde_json::Value>,
    ) -> Result<LoggingResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(include)?);
        params.push(serde_json::to_value(exclude)?);
        self.transport.call("logging", &params).await
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
    pub async fn migratewallet(
        &self,
        wallet_name: String,
        passphrase: String,
    ) -> Result<MigratewalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(wallet_name)?);
        params.push(serde_json::to_value(passphrase)?);
        self.transport.call("migratewallet", &params).await
    }

    /// Bump the scheduler into the future (-regtest only)
    pub async fn mockscheduler(&self, delta_time: u64) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(delta_time)?);
        self.transport.call("mockscheduler", &params).await
    }

    /// Requests that a ping be sent to all other nodes, to measure ping time.
    /// Results are provided in getpeerinfo.
    /// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
    pub async fn ping(&self) -> Result<(), TransportError> {
        self.transport.call("ping", &[]).await
    }

    /// Treats a block as if it were received before others with the same work.
    ///
    /// A later preciousblock call can override the effect of an earlier one.
    ///
    /// The effects of preciousblock are not retained across restarts.
    pub async fn preciousblock(&self, blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        self.transport.call("preciousblock", &params).await
    }

    /// Accepts the transaction into mined blocks at a higher (or lower) priority
    pub async fn prioritisetransaction(
        &self,
        txid: bitcoin::Txid,
        dummy: Option<String>,
        fee_delta: f64,
    ) -> Result<PrioritisetransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(dummy)?);
        params.push(serde_json::to_value(fee_delta)?);
        self.transport.call("prioritisetransaction", &params).await
    }

    /// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
    /// Requires ``-prune`` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via ``getblockfrompeer``), local deletion is irreversible.
    pub async fn pruneblockchain(
        &self,
        height: u64,
    ) -> Result<PruneblockchainResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(height)?);
        self.transport.call("pruneblockchain", &params).await
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
    pub async fn psbtbumpfee(
        &self,
        txid: bitcoin::Txid,
        options: serde_json::Value,
    ) -> Result<PsbtbumpfeeResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(options)?);
        self.transport.call("psbtbumpfee", &params).await
    }

    /// Removes invalidity status of a block, its ancestors and its descendants, reconsider them for activation.
    /// This can be used to undo the effects of invalidateblock.
    pub async fn reconsiderblock(
        &self,
        blockhash: bitcoin::BlockHash,
    ) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        self.transport.call("reconsiderblock", &params).await
    }

    /// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
    pub async fn removeprunedfunds(&self, txid: bitcoin::Txid) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        self.transport.call("removeprunedfunds", &params).await
    }

    /// Rescan the local blockchain for wallet related transactions.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    pub async fn rescanblockchain(
        &self,
        start_height: u64,
        stop_height: u64,
    ) -> Result<RescanblockchainResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(start_height)?);
        params.push(serde_json::to_value(stop_height)?);
        self.transport.call("rescanblockchain", &params).await
    }

    /// Restores and loads a wallet from backup.
    ///
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    pub async fn restorewallet(
        &self,
        wallet_name: String,
        backup_file: String,
        load_on_startup: bool,
    ) -> Result<RestorewalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(wallet_name)?);
        params.push(serde_json::to_value(backup_file)?);
        params.push(serde_json::to_value(load_on_startup)?);
        self.transport.call("restorewallet", &params).await
    }

    /// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
    pub async fn savemempool(&self) -> Result<SavemempoolResponse, TransportError> {
        self.transport.call("savemempool", &[]).await
    }

    /// Return relevant blockhashes for given descriptors (requires blockfilterindex).
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn scanblocks(
        &self,
        action: String,
        scanobjects: Vec<serde_json::Value>,
        start_height: u64,
        stop_height: u64,
        filtertype: String,
        options: serde_json::Value,
    ) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(action)?);
        params.push(serde_json::to_value(scanobjects)?);
        params.push(serde_json::to_value(start_height)?);
        params.push(serde_json::to_value(stop_height)?);
        params.push(serde_json::to_value(filtertype)?);
        params.push(serde_json::to_value(options)?);
        self.transport.call("scanblocks", &params).await
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
    pub async fn scantxoutset(
        &self,
        action: String,
        scanobjects: Vec<serde_json::Value>,
    ) -> Result<ScantxoutsetResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(action)?);
        params.push(serde_json::to_value(scanobjects)?);
        self.transport.call("scantxoutset", &params).await
    }

    /// Return RPC command JSON Schema descriptions.
    pub async fn schema(&self) -> Result<SchemaResponse, TransportError> {
        self.transport.call("schema", &[]).await
    }

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Send a transaction.
    pub async fn send(
        &self,
        outputs: Vec<serde_json::Value>,
        conf_target: u64,
        estimate_mode: String,
        fee_rate: f64,
        options: serde_json::Value,
        version: u32,
    ) -> Result<SendResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(outputs)?);
        params.push(serde_json::to_value(conf_target)?);
        params.push(serde_json::to_value(estimate_mode)?);
        params.push(serde_json::to_value(fee_rate)?);
        params.push(serde_json::to_value(options)?);
        params.push(serde_json::to_value(version)?);
        self.transport.call("send", &params).await
    }

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
    /// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
    /// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using ``send_max`` to exclude inputs that are worth less than the fees needed to spend them.
    pub async fn sendall(
        &self,
        recipients: Vec<serde_json::Value>,
        conf_target: u64,
        estimate_mode: String,
        fee_rate: f64,
        options: serde_json::Value,
    ) -> Result<SendallResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(recipients)?);
        params.push(serde_json::to_value(conf_target)?);
        params.push(serde_json::to_value(estimate_mode)?);
        params.push(serde_json::to_value(fee_rate)?);
        params.push(serde_json::to_value(options)?);
        self.transport.call("sendall", &params).await
    }

    /// Send multiple times. Amounts are double-precision floating point numbers.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sendmany(
        &self,
        dummy: Option<String>,
        amounts: serde_json::Value,
        minconf: u32,
        comment: String,
        subtractfeefrom: Vec<serde_json::Value>,
        replaceable: bool,
        conf_target: u64,
        estimate_mode: String,
        fee_rate: f64,
        verbose: bool,
    ) -> Result<SendmanyResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(dummy)?);
        params.push(serde_json::to_value(amounts)?);
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(comment)?);
        params.push(serde_json::to_value(subtractfeefrom)?);
        params.push(serde_json::to_value(replaceable)?);
        params.push(serde_json::to_value(conf_target)?);
        params.push(serde_json::to_value(estimate_mode)?);
        params.push(serde_json::to_value(fee_rate)?);
        params.push(serde_json::to_value(verbose)?);
        self.transport.call("sendmany", &params).await
    }

    /// Send a p2p message to a peer specified by id.
    /// The message type and body must be provided, the message header will be generated.
    /// This RPC is for testing only.
    pub async fn sendmsgtopeer(
        &self,
        peer_id: u64,
        msg_type: String,
        msg: String,
    ) -> Result<SendmsgtopeerResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(peer_id)?);
        params.push(serde_json::to_value(msg_type)?);
        params.push(serde_json::to_value(msg)?);
        self.transport.call("sendmsgtopeer", &params).await
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
    pub async fn sendrawtransaction(
        &self,
        hexstring: String,
        maxfeerate: f64,
        maxburnamount: f64,
    ) -> Result<SendrawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        params.push(serde_json::to_value(maxfeerate)?);
        params.push(serde_json::to_value(maxburnamount)?);
        self.transport.call("sendrawtransaction", &params).await
    }

    /// Send an amount to a given address.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sendtoaddress(
        &self,
        address: String,
        amount: bitcoin::Amount,
        comment: String,
        comment_to: String,
        subtractfeefromamount: bool,
        replaceable: bool,
        conf_target: u64,
        estimate_mode: String,
        avoid_reuse: bool,
        fee_rate: f64,
        verbose: bool,
    ) -> Result<SendtoaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(amount)?);
        params.push(serde_json::to_value(comment)?);
        params.push(serde_json::to_value(comment_to)?);
        params.push(serde_json::to_value(subtractfeefromamount)?);
        params.push(serde_json::to_value(replaceable)?);
        params.push(serde_json::to_value(conf_target)?);
        params.push(serde_json::to_value(estimate_mode)?);
        params.push(serde_json::to_value(avoid_reuse)?);
        params.push(serde_json::to_value(fee_rate)?);
        params.push(serde_json::to_value(verbose)?);
        self.transport.call("sendtoaddress", &params).await
    }

    /// Attempts to add or remove an IP/Subnet from the banned list.
    pub async fn setban(
        &self,
        subnet: String,
        command: String,
        bantime: u64,
        absolute: bool,
    ) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(subnet)?);
        params.push(serde_json::to_value(command)?);
        params.push(serde_json::to_value(bantime)?);
        params.push(serde_json::to_value(absolute)?);
        self.transport.call("setban", &params).await
    }

    /// Sets the label associated with the given address.
    pub async fn setlabel(&self, address: String, label: String) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(label)?);
        self.transport.call("setlabel", &params).await
    }

    /// Set the local time to given timestamp (-regtest only)
    pub async fn setmocktime(&self, timestamp: u64) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(timestamp)?);
        self.transport.call("setmocktime", &params).await
    }

    /// Disable/enable all p2p network activity.
    pub async fn setnetworkactive(
        &self,
        state: bool,
    ) -> Result<SetnetworkactiveResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(state)?);
        self.transport.call("setnetworkactive", &params).await
    }

    /// (DEPRECATED) Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
    /// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
    pub async fn settxfee(
        &self,
        amount: bitcoin::Amount,
    ) -> Result<SettxfeeResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(amount)?);
        self.transport.call("settxfee", &params).await
    }

    /// Change the state of the given wallet flag for a wallet.
    pub async fn setwalletflag(
        &self,
        flag: String,
        value: bool,
    ) -> Result<SetwalletflagResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(flag)?);
        params.push(serde_json::to_value(value)?);
        self.transport.call("setwalletflag", &params).await
    }

    /// Sign a message with the private key of an address
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn signmessage(
        &self,
        address: String,
        message: String,
    ) -> Result<SignmessageResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(message)?);
        self.transport.call("signmessage", &params).await
    }

    /// Sign a message with the private key of an address
    pub async fn signmessagewithprivkey(
        &self,
        privkey: String,
        message: String,
    ) -> Result<SignmessagewithprivkeyResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(privkey)?);
        params.push(serde_json::to_value(message)?);
        self.transport.call("signmessagewithprivkey", &params).await
    }

    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second argument is an array of base58-encoded private
    /// keys that will be the only keys used to sign the transaction.
    /// The third optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    pub async fn signrawtransactionwithkey(
        &self,
        hexstring: String,
        privkeys: Vec<String>,
        prevtxs: Vec<serde_json::Value>,
        sighashtype: String,
    ) -> Result<SignrawtransactionwithkeyResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        params.push(serde_json::to_value(privkeys)?);
        params.push(serde_json::to_value(prevtxs)?);
        params.push(serde_json::to_value(sighashtype)?);
        self.transport.call("signrawtransactionwithkey", &params).await
    }

    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn signrawtransactionwithwallet(
        &self,
        hexstring: String,
        prevtxs: Vec<serde_json::Value>,
        sighashtype: String,
    ) -> Result<SignrawtransactionwithwalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        params.push(serde_json::to_value(prevtxs)?);
        params.push(serde_json::to_value(sighashtype)?);
        self.transport.call("signrawtransactionwithwallet", &params).await
    }

    /// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
    pub async fn simulaterawtransaction(
        &self,
        rawtxs: Vec<serde_json::Value>,
        options: serde_json::Value,
    ) -> Result<SimulaterawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(rawtxs)?);
        params.push(serde_json::to_value(options)?);
        self.transport.call("simulaterawtransaction", &params).await
    }

    /// Request a graceful shutdown of Bitcoin Core.
    pub async fn stop(&self, wait: u64) -> Result<StopResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(wait)?);
        self.transport.call("stop", &params).await
    }

    /// Attempts to submit new block to network.
    /// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
    pub async fn submitblock(
        &self,
        hexdata: String,
        dummy: Option<String>,
    ) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexdata)?);
        params.push(serde_json::to_value(dummy)?);
        self.transport.call("submitblock", &params).await
    }

    /// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
    /// Throws when the header is invalid.
    pub async fn submitheader(&self, hexdata: String) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexdata)?);
        self.transport.call("submitheader", &params).await
    }

    /// Submit a package of raw transactions (serialized, hex-encoded) to local node.
    /// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
    /// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
    /// Warning: successful submission does not mean the transactions will propagate throughout the network.
    pub async fn submitpackage(
        &self,
        package: Vec<serde_json::Value>,
        maxfeerate: f64,
        maxburnamount: f64,
    ) -> Result<SubmitpackageResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(package)?);
        params.push(serde_json::to_value(maxfeerate)?);
        params.push(serde_json::to_value(maxburnamount)?);
        self.transport.call("submitpackage", &params).await
    }

    /// Waits for the validation interface queue to catch up on everything that was there when we entered this function.
    pub async fn syncwithvalidationinterfacequeue(&self) -> Result<(), TransportError> {
        self.transport.call("syncwithvalidationinterfacequeue", &[]).await
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
    pub async fn testmempoolaccept(
        &self,
        rawtxs: Vec<serde_json::Value>,
        maxfeerate: f64,
    ) -> Result<TestmempoolacceptResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(rawtxs)?);
        params.push(serde_json::to_value(maxfeerate)?);
        self.transport.call("testmempoolaccept", &params).await
    }

    /// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
    /// If both are specified, they must be identical.
    pub async fn unloadwallet(
        &self,
        wallet_name: String,
        load_on_startup: bool,
    ) -> Result<UnloadwalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(wallet_name)?);
        params.push(serde_json::to_value(load_on_startup)?);
        self.transport.call("unloadwallet", &params).await
    }

    /// Returns the total uptime of the server.
    pub async fn uptime(&self) -> Result<UptimeResponse, TransportError> {
        self.transport.call("uptime", &[]).await
    }

    /// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
    pub async fn utxoupdatepsbt(
        &self,
        psbt: String,
        descriptors: Vec<serde_json::Value>,
    ) -> Result<UtxoupdatepsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(psbt)?);
        params.push(serde_json::to_value(descriptors)?);
        self.transport.call("utxoupdatepsbt", &params).await
    }

    /// Return information about the given bitcoin address.
    pub async fn validateaddress(
        &self,
        address: String,
    ) -> Result<ValidateaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        self.transport.call("validateaddress", &params).await
    }

    /// Verifies blockchain database.
    pub async fn verifychain(
        &self,
        checklevel: u32,
        nblocks: u64,
    ) -> Result<VerifychainResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(checklevel)?);
        params.push(serde_json::to_value(nblocks)?);
        self.transport.call("verifychain", &params).await
    }

    /// Verify a signed message.
    pub async fn verifymessage(
        &self,
        address: String,
        signature: String,
        message: String,
    ) -> Result<VerifymessageResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(signature)?);
        params.push(serde_json::to_value(message)?);
        self.transport.call("verifymessage", &params).await
    }

    /// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
    /// and throwing an RPC error if the block is not in our best chain
    pub async fn verifytxoutproof(
        &self,
        proof: String,
    ) -> Result<VerifytxoutproofResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(proof)?);
        self.transport.call("verifytxoutproof", &params).await
    }

    /// Waits for a specific new block and returns useful info about it.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn waitforblock(
        &self,
        blockhash: bitcoin::BlockHash,
        timeout: u64,
    ) -> Result<WaitforblockResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        params.push(serde_json::to_value(timeout)?);
        self.transport.call("waitforblock", &params).await
    }

    /// Waits for (at least) block height and returns the height and hash
    /// of the current tip.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn waitforblockheight(
        &self,
        height: u64,
        timeout: u64,
    ) -> Result<WaitforblockheightResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(height)?);
        params.push(serde_json::to_value(timeout)?);
        self.transport.call("waitforblockheight", &params).await
    }

    /// Waits for any new block and returns useful info about it.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn waitfornewblock(
        &self,
        timeout: u64,
        current_tip: String,
    ) -> Result<WaitfornewblockResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(timeout)?);
        params.push(serde_json::to_value(current_tip)?);
        self.transport.call("waitfornewblock", &params).await
    }

    /// Creates and funds a transaction in the Partially Signed Transaction format.
    /// Implements the Creator and Updater roles.
    /// All existing inputs must either have their previous output transaction be in the wallet
    /// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
    pub async fn walletcreatefundedpsbt(
        &self,
        inputs: Vec<serde_json::Value>,
        outputs: Vec<serde_json::Value>,
        locktime: u32,
        options: serde_json::Value,
        bip32derivs: bool,
        version: u32,
    ) -> Result<WalletcreatefundedpsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(inputs)?);
        params.push(serde_json::to_value(outputs)?);
        params.push(serde_json::to_value(locktime)?);
        params.push(serde_json::to_value(options)?);
        params.push(serde_json::to_value(bip32derivs)?);
        params.push(serde_json::to_value(version)?);
        self.transport.call("walletcreatefundedpsbt", &params).await
    }

    /// Display address on an external signer for verification.
    pub async fn walletdisplayaddress(
        &self,
        address: String,
    ) -> Result<WalletdisplayaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        self.transport.call("walletdisplayaddress", &params).await
    }

    /// Removes the wallet encryption key from memory, locking the wallet.
    /// After calling this method, you will need to call walletpassphrase again
    /// before being able to call any methods which require the wallet to be unlocked.
    pub async fn walletlock(&self) -> Result<(), TransportError> {
        self.transport.call("walletlock", &[]).await
    }

    /// Stores the wallet decryption key in memory for "timeout" seconds.
    /// This is needed prior to performing transactions related to private keys such as sending bitcoins
    ///
    /// Note:
    /// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
    /// time that overrides the old one.
    pub async fn walletpassphrase(
        &self,
        passphrase: String,
        timeout: u64,
    ) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(passphrase)?);
        params.push(serde_json::to_value(timeout)?);
        self.transport.call("walletpassphrase", &params).await
    }

    /// Changes the wallet passphrase from "oldpassphrase" to "newpassphrase".
    pub async fn walletpassphrasechange(
        &self,
        oldpassphrase: String,
        newpassphrase: String,
    ) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(oldpassphrase)?);
        params.push(serde_json::to_value(newpassphrase)?);
        self.transport.call("walletpassphrasechange", &params).await
    }

    /// Update a PSBT with input information from our wallet and then sign inputs
    /// that we can sign for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn walletprocesspsbt(
        &self,
        psbt: String,
        sign: bool,
        sighashtype: String,
        bip32derivs: bool,
        finalize: bool,
    ) -> Result<WalletprocesspsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(psbt)?);
        params.push(serde_json::to_value(sign)?);
        params.push(serde_json::to_value(sighashtype)?);
        params.push(serde_json::to_value(bip32derivs)?);
        params.push(serde_json::to_value(finalize)?);
        self.transport.call("walletprocesspsbt", &params).await
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
        Ok(serde_json::to_value(
            self.sendtoaddress(
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
            )
            .await?,
        )?)
    }

    pub async fn send_to_address_with_fee_rate(
        &self,
        address: String,
        amount: Amount,
        fee_rate: f64,
    ) -> Result<Value, TransportError> {
        Ok(serde_json::to_value(
            self.sendtoaddress(
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
            )
            .await?,
        )?)
    }
}

impl Drop for BitcoinTestClient {
    fn drop(&mut self) { let _ = self.node_manager.take(); }
}
