// Generated client trait for Bitcoin Core v30

use std::future::Future;

use async_trait::async_trait;
use bitcoin_rpc_types::HashOrHeight;
use serde::de::DeserializeOwned;
use serde::ser::SerializeSeq;
use serde::Deserialize;

use crate::responses::*;
use crate::transport::{TransportError, TransportExt, TransportTrait};

#[derive(Debug, Clone, Deserialize)]
pub struct PrioritisetransactionParams {
    pub _txid: bitcoin::Txid,
    pub _fee_delta: f64,
    pub _dummy: Option<String>,
}

impl serde::Serialize for PrioritisetransactionParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self._txid)?;
        seq.serialize_element(&self._dummy)?;
        seq.serialize_element(&self._fee_delta)?;
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendmanyParams {
    pub _amounts: serde_json::Value,
    pub _dummy: Option<String>,
    pub _minconf: Option<u32>,
    pub _comment: Option<String>,
    pub _subtractfeefrom: Option<Vec<serde_json::Value>>,
    pub _replaceable: Option<bool>,
    pub _conf_target: Option<u64>,
    pub _estimate_mode: Option<String>,
    pub _fee_rate: Option<f64>,
    pub _verbose: Option<bool>,
}

impl serde::Serialize for SendmanyParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(10))?;
        seq.serialize_element(&self._dummy)?;
        seq.serialize_element(&self._amounts)?;
        seq.serialize_element(&self._minconf)?;
        seq.serialize_element(&self._comment)?;
        seq.serialize_element(&self._subtractfeefrom)?;
        seq.serialize_element(&self._replaceable)?;
        seq.serialize_element(&self._conf_target)?;
        seq.serialize_element(&self._estimate_mode)?;
        seq.serialize_element(&self._fee_rate)?;
        seq.serialize_element(&self._verbose)?;
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletcreatefundedpsbtParams {
    pub _outputs: Vec<serde_json::Value>,
    pub _inputs: Option<Vec<serde_json::Value>>,
    pub _locktime: Option<u32>,
    pub _options: Option<serde_json::Value>,
    pub _bip32derivs: Option<bool>,
    pub _version: Option<u32>,
}

impl serde::Serialize for WalletcreatefundedpsbtParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(6))?;
        seq.serialize_element(&self._inputs)?;
        seq.serialize_element(&self._outputs)?;
        seq.serialize_element(&self._locktime)?;
        seq.serialize_element(&self._options)?;
        seq.serialize_element(&self._bip32derivs)?;
        seq.serialize_element(&self._version)?;
        seq.end()
    }
}

#[doc = r#"A versioned client trait for Bitcoin Core v30"#]
#[async_trait]
pub trait BitcoinClientV30: Send + Sync + TransportTrait + TransportExt + RpcDispatchExt {
    /// Mark in-wallet transaction <txid> as abandoned
    /// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
    /// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
    /// It only works on transactions which are not included in a block and are not currently in the mempool.
    /// It has no effect on transactions which are already abandoned.
    async fn abandontransaction(&self, _txid: bitcoin::Txid) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_txid)];
        self.dispatch_json::<()>("abandontransaction", &params).await
    }

    /// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    async fn abortrescan(&self) -> Result<AbortrescanResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<AbortrescanResponse>("abortrescan", &params).await
    }

    /// Open an outbound connection to a specified node. This RPC is for testing only.
    async fn addconnection(
        &self,
        _address: String,
        _connection_type: String,
        _v2transport: bool,
    ) -> Result<AddconnectionResponse, TransportError> {
        let params = vec![
            serde_json::json!(_address),
            serde_json::json!(_connection_type),
            serde_json::json!(_v2transport),
        ];
        self.dispatch_json::<AddconnectionResponse>("addconnection", &params).await
    }

    /// Attempts to add or remove a node from the addnode list.
    /// Or try a connection to a node once.
    /// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
    /// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
    /// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
    async fn addnode(
        &self,
        _node: String,
        _command: String,
        _v2transport: Option<bool>,
    ) -> Result<(), TransportError> {
        let params = vec![
            serde_json::json!(_node),
            serde_json::json!(_command),
            serde_json::json!(_v2transport),
        ];
        self.dispatch_json::<()>("addnode", &params).await
    }

    /// Add the address of a potential peer to an address manager table. This RPC is for testing only.
    async fn addpeeraddress(
        &self,
        _address: String,
        _port: u16,
        _tried: Option<bool>,
    ) -> Result<AddpeeraddressResponse, TransportError> {
        let params =
            vec![serde_json::json!(_address), serde_json::json!(_port), serde_json::json!(_tried)];
        self.dispatch_json::<AddpeeraddressResponse>("addpeeraddress", &params).await
    }

    /// Analyzes and provides information about the current status of a PSBT and its inputs
    async fn analyzepsbt(&self, _psbt: String) -> Result<AnalyzepsbtResponse, TransportError> {
        let params = vec![serde_json::json!(_psbt)];
        self.dispatch_json::<AnalyzepsbtResponse>("analyzepsbt", &params).await
    }

    /// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
    async fn backupwallet(&self, _destination: String) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_destination)];
        self.dispatch_json::<()>("backupwallet", &params).await
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
    async fn bumpfee(
        &self,
        _txid: bitcoin::Txid,
        _options: Option<serde_json::Value>,
    ) -> Result<BumpfeeResponse, TransportError> {
        let params = vec![serde_json::json!(_txid), serde_json::json!(_options)];
        self.dispatch_json::<BumpfeeResponse>("bumpfee", &params).await
    }

    /// Clear all banned IPs.
    async fn clearbanned(&self) -> Result<(), TransportError> {
        let params = vec![];
        self.dispatch_json::<()>("clearbanned", &params).await
    }

    /// Combine multiple partially signed Bitcoin transactions into one transaction.
    /// Implements the Combiner role.
    async fn combinepsbt(
        &self,
        _txs: Vec<serde_json::Value>,
    ) -> Result<CombinepsbtResponse, TransportError> {
        let params = vec![serde_json::json!(_txs)];
        self.dispatch_json::<CombinepsbtResponse>("combinepsbt", &params).await
    }

    /// Combine multiple partially signed transactions into one transaction.
    /// The combined transaction may be another partially signed transaction or a
    /// fully signed transaction.
    async fn combinerawtransaction(
        &self,
        _txs: Vec<serde_json::Value>,
    ) -> Result<CombinerawtransactionResponse, TransportError> {
        let params = vec![serde_json::json!(_txs)];
        self.dispatch_json::<CombinerawtransactionResponse>("combinerawtransaction", &params).await
    }

    /// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
    /// createpsbt and walletcreatefundedpsbt should be used for new applications.
    async fn converttopsbt(
        &self,
        _hexstring: String,
        _permitsigdata: Option<bool>,
        _iswitness: Option<bool>,
    ) -> Result<ConverttopsbtResponse, TransportError> {
        let params = vec![
            serde_json::json!(_hexstring),
            serde_json::json!(_permitsigdata),
            serde_json::json!(_iswitness),
        ];
        self.dispatch_json::<ConverttopsbtResponse>("converttopsbt", &params).await
    }

    /// Creates a multi-signature address with n signatures of m keys required.
    /// It returns a json object with the address and redeemScript.
    async fn createmultisig(
        &self,
        _nrequired: u32,
        _keys: Vec<String>,
        _address_type: Option<String>,
    ) -> Result<CreatemultisigResponse, TransportError> {
        let params = vec![
            serde_json::json!(_nrequired),
            serde_json::json!(_keys),
            serde_json::json!(_address_type),
        ];
        self.dispatch_json::<CreatemultisigResponse>("createmultisig", &params).await
    }

    /// Creates a transaction in the Partially Signed Transaction format.
    /// Implements the Creator role.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    async fn createpsbt(
        &self,
        _inputs: Vec<serde_json::Value>,
        _outputs: Vec<serde_json::Value>,
        _locktime: Option<u32>,
        _replaceable: Option<bool>,
        _version: Option<u32>,
    ) -> Result<CreatepsbtResponse, TransportError> {
        let params = vec![
            serde_json::json!(_inputs),
            serde_json::json!(_outputs),
            serde_json::json!(_locktime),
            serde_json::json!(_replaceable),
            serde_json::json!(_version),
        ];
        self.dispatch_json::<CreatepsbtResponse>("createpsbt", &params).await
    }

    /// Create a transaction spending the given inputs and creating new outputs.
    /// Outputs can be addresses or data.
    /// Returns hex-encoded raw transaction.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    async fn createrawtransaction(
        &self,
        _inputs: Vec<serde_json::Value>,
        _outputs: Vec<serde_json::Value>,
        _locktime: Option<u32>,
        _replaceable: Option<bool>,
        _version: Option<u32>,
    ) -> Result<CreaterawtransactionResponse, TransportError> {
        let params = vec![
            serde_json::json!(_inputs),
            serde_json::json!(_outputs),
            serde_json::json!(_locktime),
            serde_json::json!(_replaceable),
            serde_json::json!(_version),
        ];
        self.dispatch_json::<CreaterawtransactionResponse>("createrawtransaction", &params).await
    }

    /// Creates and loads a new wallet.
    #[allow(clippy::too_many_arguments)]
    async fn createwallet(
        &self,
        _wallet_name: String,
        _disable_private_keys: Option<bool>,
        _blank: Option<bool>,
        _passphrase: Option<String>,
        _avoid_reuse: Option<bool>,
        _descriptors: Option<bool>,
        _load_on_startup: Option<bool>,
        _external_signer: Option<bool>,
    ) -> Result<CreatewalletResponse, TransportError> {
        let params = vec![
            serde_json::json!(_wallet_name),
            serde_json::json!(_disable_private_keys),
            serde_json::json!(_blank),
            serde_json::json!(_passphrase),
            serde_json::json!(_avoid_reuse),
            serde_json::json!(_descriptors),
            serde_json::json!(_load_on_startup),
            serde_json::json!(_external_signer),
        ];
        self.dispatch_json::<CreatewalletResponse>("createwallet", &params).await
    }

    /// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn createwalletdescriptor(
        &self,
        r#_type: String,
        _options: Option<serde_json::Value>,
    ) -> Result<CreatewalletdescriptorResponse, TransportError> {
        let params = vec![serde_json::json!(r#_type), serde_json::json!(_options)];
        self.dispatch_json::<CreatewalletdescriptorResponse>("createwalletdescriptor", &params)
            .await
    }

    /// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
    async fn decodepsbt(&self, _psbt: String) -> Result<DecodepsbtResponse, TransportError> {
        let params = vec![serde_json::json!(_psbt)];
        self.dispatch_json::<DecodepsbtResponse>("decodepsbt", &params).await
    }

    /// Return a JSON object representing the serialized, hex-encoded transaction.
    async fn decoderawtransaction(
        &self,
        _hexstring: String,
        _iswitness: Option<bool>,
    ) -> Result<DecoderawtransactionResponse, TransportError> {
        let params = vec![serde_json::json!(_hexstring), serde_json::json!(_iswitness)];
        self.dispatch_json::<DecoderawtransactionResponse>("decoderawtransaction", &params).await
    }

    /// Decode a hex-encoded script.
    async fn decodescript(
        &self,
        _hexstring: String,
    ) -> Result<DecodescriptResponse, TransportError> {
        let params = vec![serde_json::json!(_hexstring)];
        self.dispatch_json::<DecodescriptResponse>("decodescript", &params).await
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
    async fn deriveaddresses(
        &self,
        _descriptor: String,
        _range: Option<serde_json::Value>,
    ) -> Result<DeriveaddressesResponse, TransportError> {
        let params = vec![serde_json::json!(_descriptor), serde_json::json!(_range)];
        self.dispatch_json::<DeriveaddressesResponse>("deriveaddresses", &params).await
    }

    /// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
    /// Then, sign the inputs we are able to with information from the output descriptors.
    async fn descriptorprocesspsbt(
        &self,
        _psbt: String,
        _descriptors: Vec<serde_json::Value>,
        _sighashtype: Option<String>,
        _bip32derivs: Option<bool>,
        _finalize: Option<bool>,
    ) -> Result<DescriptorprocesspsbtResponse, TransportError> {
        let params = vec![
            serde_json::json!(_psbt),
            serde_json::json!(_descriptors),
            serde_json::json!(_sighashtype),
            serde_json::json!(_bip32derivs),
            serde_json::json!(_finalize),
        ];
        self.dispatch_json::<DescriptorprocesspsbtResponse>("descriptorprocesspsbt", &params).await
    }

    /// Immediately disconnects from the specified peer node.
    ///
    /// Strictly one out of 'address' and 'nodeid' can be provided to identify the node.
    ///
    /// To disconnect by nodeid, either set 'address' to the empty string, or call using the named 'nodeid' argument only.
    async fn disconnectnode(
        &self,
        _address: Option<String>,
        _nodeid: Option<u64>,
    ) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_address), serde_json::json!(_nodeid)];
        self.dispatch_json::<()>("disconnectnode", &params).await
    }

    /// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
    ///
    /// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
    ///
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn dumptxoutset(
        &self,
        _path: String,
        r#_type: Option<String>,
        _options: Option<serde_json::Value>,
    ) -> Result<DumptxoutsetResponse, TransportError> {
        let params =
            vec![serde_json::json!(_path), serde_json::json!(r#_type), serde_json::json!(_options)];
        self.dispatch_json::<DumptxoutsetResponse>("dumptxoutset", &params).await
    }

    /// Simply echo back the input arguments. This command is for testing.
    ///
    /// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
    ///
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    #[allow(clippy::too_many_arguments)]
    async fn echo(
        &self,
        _arg0: Option<String>,
        _arg1: Option<String>,
        _arg2: Option<String>,
        _arg3: Option<String>,
        _arg4: Option<String>,
        _arg5: Option<String>,
        _arg6: Option<String>,
        _arg7: Option<String>,
        _arg8: Option<String>,
        _arg9: Option<String>,
    ) -> Result<EchoResponse, TransportError> {
        let params = vec![
            serde_json::json!(_arg0),
            serde_json::json!(_arg1),
            serde_json::json!(_arg2),
            serde_json::json!(_arg3),
            serde_json::json!(_arg4),
            serde_json::json!(_arg5),
            serde_json::json!(_arg6),
            serde_json::json!(_arg7),
            serde_json::json!(_arg8),
            serde_json::json!(_arg9),
        ];
        self.dispatch_json::<EchoResponse>("echo", &params).await
    }

    /// Echo back the input argument, passing it through a spawned process in a multiprocess build.
    /// This command is for testing.
    async fn echoipc(&self, _arg: String) -> Result<EchoipcResponse, TransportError> {
        let params = vec![serde_json::json!(_arg)];
        self.dispatch_json::<EchoipcResponse>("echoipc", &params).await
    }

    /// Simply echo back the input arguments. This command is for testing.
    ///
    /// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
    ///
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    #[allow(clippy::too_many_arguments)]
    async fn echojson(
        &self,
        _arg0: Option<String>,
        _arg1: Option<String>,
        _arg2: Option<String>,
        _arg3: Option<String>,
        _arg4: Option<String>,
        _arg5: Option<String>,
        _arg6: Option<String>,
        _arg7: Option<String>,
        _arg8: Option<String>,
        _arg9: Option<String>,
    ) -> Result<EchojsonResponse, TransportError> {
        let params = vec![
            serde_json::json!(_arg0),
            serde_json::json!(_arg1),
            serde_json::json!(_arg2),
            serde_json::json!(_arg3),
            serde_json::json!(_arg4),
            serde_json::json!(_arg5),
            serde_json::json!(_arg6),
            serde_json::json!(_arg7),
            serde_json::json!(_arg8),
            serde_json::json!(_arg9),
        ];
        self.dispatch_json::<EchojsonResponse>("echojson", &params).await
    }

    /// Encrypts the wallet with 'passphrase'. This is for first time encryption.
    /// After this, any calls that interact with private keys such as sending or signing
    /// will require the passphrase to be set prior to making these calls.
    /// Use the walletpassphrase call for this, and then walletlock call.
    /// If the wallet is already encrypted, use the walletpassphrasechange call.
    /// ** IMPORTANT **
    /// For security reasons, the encryption process will generate a new HD seed, resulting
    /// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
    /// securely back up the newly generated wallet file using the backupwallet RPC.
    async fn encryptwallet(
        &self,
        _passphrase: String,
    ) -> Result<EncryptwalletResponse, TransportError> {
        let params = vec![serde_json::json!(_passphrase)];
        self.dispatch_json::<EncryptwalletResponse>("encryptwallet", &params).await
    }

    /// Returns a list of external signers from -signer.
    async fn enumeratesigners(&self) -> Result<EnumeratesignersResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<EnumeratesignersResponse>("enumeratesigners", &params).await
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
    async fn estimaterawfee(
        &self,
        _conf_target: u64,
        _threshold: Option<u64>,
    ) -> Result<EstimaterawfeeResponse, TransportError> {
        let params = vec![serde_json::json!(_conf_target), serde_json::json!(_threshold)];
        self.dispatch_json::<EstimaterawfeeResponse>("estimaterawfee", &params).await
    }

    /// Estimates the approximate fee per kilobyte needed for a transaction to begin
    /// confirmation within conf_target blocks if possible and return the number of blocks
    /// for which the estimate is valid. Uses virtual transaction size as defined
    /// in BIP 141 (witness data is discounted).
    async fn estimatesmartfee(
        &self,
        _conf_target: u64,
        _estimate_mode: Option<String>,
    ) -> Result<EstimatesmartfeeResponse, TransportError> {
        let params = vec![serde_json::json!(_conf_target), serde_json::json!(_estimate_mode)];
        self.dispatch_json::<EstimatesmartfeeResponse>("estimatesmartfee", &params).await
    }

    /// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
    /// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
    /// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
    /// Implements the Finalizer and Extractor roles.
    async fn finalizepsbt(
        &self,
        _psbt: String,
        _extract: Option<bool>,
    ) -> Result<FinalizepsbtResponse, TransportError> {
        let params = vec![serde_json::json!(_psbt), serde_json::json!(_extract)];
        self.dispatch_json::<FinalizepsbtResponse>("finalizepsbt", &params).await
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
    async fn fundrawtransaction(
        &self,
        _hexstring: String,
        _options: Option<serde_json::Value>,
        _iswitness: Option<bool>,
    ) -> Result<FundrawtransactionResponse, TransportError> {
        let params = vec![
            serde_json::json!(_hexstring),
            serde_json::json!(_options),
            serde_json::json!(_iswitness),
        ];
        self.dispatch_json::<FundrawtransactionResponse>("fundrawtransaction", &params).await
    }

    /// has been replaced by the -generate cli option. Refer to -help for more information.
    async fn generate(&self) -> Result<(), TransportError> {
        let params = vec![];
        self.dispatch_json::<()>("generate", &params).await
    }

    /// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
    async fn generateblock(
        &self,
        _output: String,
        _transactions: Vec<serde_json::Value>,
        _submit: Option<bool>,
    ) -> Result<GenerateblockResponse, TransportError> {
        let params = vec![
            serde_json::json!(_output),
            serde_json::json!(_transactions),
            serde_json::json!(_submit),
        ];
        self.dispatch_json::<GenerateblockResponse>("generateblock", &params).await
    }

    /// Mine to a specified address and return the block hashes.
    async fn generatetoaddress(
        &self,
        _nblocks: u64,
        _address: String,
        _maxtries: Option<u64>,
    ) -> Result<GeneratetoaddressResponse, TransportError> {
        let params = vec![
            serde_json::json!(_nblocks),
            serde_json::json!(_address),
            serde_json::json!(_maxtries),
        ];
        self.dispatch_json::<GeneratetoaddressResponse>("generatetoaddress", &params).await
    }

    /// Mine to a specified descriptor and return the block hashes.
    async fn generatetodescriptor(
        &self,
        _num_blocks: u64,
        _descriptor: String,
        _maxtries: Option<u64>,
    ) -> Result<GeneratetodescriptorResponse, TransportError> {
        let params = vec![
            serde_json::json!(_num_blocks),
            serde_json::json!(_descriptor),
            serde_json::json!(_maxtries),
        ];
        self.dispatch_json::<GeneratetodescriptorResponse>("generatetodescriptor", &params).await
    }

    /// Returns information about the given added node, or all added nodes
    /// (note that onetry addnodes are not listed here)
    async fn getaddednodeinfo(
        &self,
        _node: Option<String>,
    ) -> Result<GetaddednodeinfoResponse, TransportError> {
        let params = vec![serde_json::json!(_node)];
        self.dispatch_json::<GetaddednodeinfoResponse>("getaddednodeinfo", &params).await
    }

    /// Returns the list of addresses assigned the specified label.
    async fn getaddressesbylabel(
        &self,
        _label: String,
    ) -> Result<GetaddressesbylabelResponse, TransportError> {
        let params = vec![serde_json::json!(_label)];
        self.dispatch_json::<GetaddressesbylabelResponse>("getaddressesbylabel", &params).await
    }

    /// Return information about the given bitcoin address.
    /// Some of the information will only be present if the address is in the active wallet.
    async fn getaddressinfo(
        &self,
        _address: String,
    ) -> Result<GetaddressinfoResponse, TransportError> {
        let params = vec![serde_json::json!(_address)];
        self.dispatch_json::<GetaddressinfoResponse>("getaddressinfo", &params).await
    }

    /// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
    async fn getaddrmaninfo(&self) -> Result<GetaddrmaninfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetaddrmaninfoResponse>("getaddrmaninfo", &params).await
    }

    /// Returns the total available balance.
    /// The available balance is what the wallet considers currently spendable, and is
    /// thus affected by options which limit spendability such as -spendzeroconfchange.
    async fn getbalance(
        &self,
        _dummy: Option<String>,
        _minconf: Option<u32>,
        _include_watchonly: Option<bool>,
        _avoid_reuse: Option<bool>,
    ) -> Result<GetbalanceResponse, TransportError> {
        let params = vec![
            serde_json::json!(_dummy),
            serde_json::json!(_minconf),
            serde_json::json!(_include_watchonly),
            serde_json::json!(_avoid_reuse),
        ];
        self.dispatch_json::<GetbalanceResponse>("getbalance", &params).await
    }

    /// Returns an object with all balances in BTC.
    async fn getbalances(&self) -> Result<GetbalancesResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetbalancesResponse>("getbalances", &params).await
    }

    /// Returns the hash of the best (tip) block in the most-work fully-validated chain.
    async fn getbestblockhash(&self) -> Result<GetbestblockhashResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetbestblockhashResponse>("getbestblockhash", &params).await
    }

    /// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
    /// If verbosity is 1, returns an Object with information about block <hash>.
    /// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
    /// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
    async fn getblock(
        &self,
        _blockhash: bitcoin::BlockHash,
        _verbosity: Option<u32>,
    ) -> Result<GetblockResponse, TransportError> {
        let params = vec![serde_json::json!(_blockhash), serde_json::json!(_verbosity)];
        self.dispatch_json::<GetblockResponse>("getblock", &params).await
    }

    /// Returns an object containing various state info regarding blockchain processing.
    async fn getblockchaininfo(&self) -> Result<GetblockchaininfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetblockchaininfoResponse>("getblockchaininfo", &params).await
    }

    /// Returns the height of the most-work fully-validated chain.
    /// The genesis block has height 0.
    async fn getblockcount(&self) -> Result<GetblockcountResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetblockcountResponse>("getblockcount", &params).await
    }

    /// Retrieve a BIP 157 content filter for a particular block.
    async fn getblockfilter(
        &self,
        _blockhash: bitcoin::BlockHash,
        _filtertype: Option<String>,
    ) -> Result<GetblockfilterResponse, TransportError> {
        let params = vec![serde_json::json!(_blockhash), serde_json::json!(_filtertype)];
        self.dispatch_json::<GetblockfilterResponse>("getblockfilter", &params).await
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
    async fn getblockfrompeer(
        &self,
        _blockhash: bitcoin::BlockHash,
        _peer_id: u64,
    ) -> Result<GetblockfrompeerResponse, TransportError> {
        let params = vec![serde_json::json!(_blockhash), serde_json::json!(_peer_id)];
        self.dispatch_json::<GetblockfrompeerResponse>("getblockfrompeer", &params).await
    }

    /// Returns hash of block in best-block-chain at height provided.
    async fn getblockhash(&self, _height: u64) -> Result<GetblockhashResponse, TransportError> {
        let params = vec![serde_json::json!(_height)];
        self.dispatch_json::<GetblockhashResponse>("getblockhash", &params).await
    }

    /// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
    /// If verbose is true, returns an Object with information about blockheader <hash>.
    async fn getblockheader(
        &self,
        _blockhash: bitcoin::BlockHash,
        _verbose: Option<bool>,
    ) -> Result<GetblockheaderResponse, TransportError> {
        let params = vec![serde_json::json!(_blockhash), serde_json::json!(_verbose)];
        self.dispatch_json::<GetblockheaderResponse>("getblockheader", &params).await
    }

    /// Compute per block statistics for a given window. All amounts are in satoshis.
    /// It won't work for some heights with pruning.
    async fn getblockstats(
        &self,
        _hash_or_height: HashOrHeight,
        _stats: Option<Vec<String>>,
    ) -> Result<GetblockstatsResponse, TransportError> {
        let params = vec![serde_json::json!(_hash_or_height), serde_json::json!(_stats)];
        self.dispatch_json::<GetblockstatsResponse>("getblockstats", &params).await
    }

    /// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
    /// It returns data needed to construct a block to work on.
    /// For full specification, see BIPs 22, 23, 9, and 145:
    /// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
    /// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
    /// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
    /// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
    async fn getblocktemplate(
        &self,
        _template_request: serde_json::Value,
    ) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_template_request)];
        self.dispatch_json::<()>("getblocktemplate", &params).await
    }

    /// Return information about chainstates.
    async fn getchainstates(&self) -> Result<GetchainstatesResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetchainstatesResponse>("getchainstates", &params).await
    }

    /// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
    async fn getchaintips(&self) -> Result<GetchaintipsResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetchaintipsResponse>("getchaintips", &params).await
    }

    /// Compute statistics about the total number and rate of transactions in the chain.
    async fn getchaintxstats(
        &self,
        _nblocks: Option<u64>,
        _blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetchaintxstatsResponse, TransportError> {
        let params = vec![serde_json::json!(_nblocks), serde_json::json!(_blockhash)];
        self.dispatch_json::<GetchaintxstatsResponse>("getchaintxstats", &params).await
    }

    /// Returns the number of connections to other nodes.
    async fn getconnectioncount(&self) -> Result<GetconnectioncountResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetconnectioncountResponse>("getconnectioncount", &params).await
    }

    /// Returns an object containing various state info regarding deployments of consensus changes.
    async fn getdeploymentinfo(
        &self,
        _blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetdeploymentinfoResponse, TransportError> {
        let params = vec![serde_json::json!(_blockhash)];
        self.dispatch_json::<GetdeploymentinfoResponse>("getdeploymentinfo", &params).await
    }

    /// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
    /// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn getdescriptoractivity(
        &self,
        _blockhashes: Vec<serde_json::Value>,
        _scanobjects: Vec<serde_json::Value>,
        _include_mempool: Option<bool>,
    ) -> Result<GetdescriptoractivityResponse, TransportError> {
        let params = vec![
            serde_json::json!(_blockhashes),
            serde_json::json!(_scanobjects),
            serde_json::json!(_include_mempool),
        ];
        self.dispatch_json::<GetdescriptoractivityResponse>("getdescriptoractivity", &params).await
    }

    /// Analyses a descriptor.
    async fn getdescriptorinfo(
        &self,
        _descriptor: String,
    ) -> Result<GetdescriptorinfoResponse, TransportError> {
        let params = vec![serde_json::json!(_descriptor)];
        self.dispatch_json::<GetdescriptorinfoResponse>("getdescriptorinfo", &params).await
    }

    /// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
    async fn getdifficulty(&self) -> Result<GetdifficultyResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetdifficultyResponse>("getdifficulty", &params).await
    }

    /// List all BIP 32 HD keys in the wallet and which descriptors use them.
    async fn gethdkeys(
        &self,
        _options: Option<serde_json::Value>,
    ) -> Result<GethdkeysResponse, TransportError> {
        let params = vec![serde_json::json!(_options)];
        self.dispatch_json::<GethdkeysResponse>("gethdkeys", &params).await
    }

    /// Returns the status of one or all available indices currently running in the node.
    async fn getindexinfo(
        &self,
        _index_name: Option<String>,
    ) -> Result<GetindexinfoResponse, TransportError> {
        let params = vec![serde_json::json!(_index_name)];
        self.dispatch_json::<GetindexinfoResponse>("getindexinfo", &params).await
    }

    /// Returns an object containing information about memory usage.
    async fn getmemoryinfo(
        &self,
        _mode: Option<String>,
    ) -> Result<GetmemoryinfoResponse, TransportError> {
        let params = vec![serde_json::json!(_mode)];
        self.dispatch_json::<GetmemoryinfoResponse>("getmemoryinfo", &params).await
    }

    /// If txid is in the mempool, returns all in-mempool ancestors.
    async fn getmempoolancestors(
        &self,
        _txid: bitcoin::Txid,
        _verbose: Option<bool>,
    ) -> Result<GetmempoolancestorsResponse, TransportError> {
        let params = vec![serde_json::json!(_txid), serde_json::json!(_verbose)];
        self.dispatch_json::<GetmempoolancestorsResponse>("getmempoolancestors", &params).await
    }

    /// If txid is in the mempool, returns all in-mempool descendants.
    async fn getmempooldescendants(
        &self,
        _txid: bitcoin::Txid,
        _verbose: Option<bool>,
    ) -> Result<GetmempooldescendantsResponse, TransportError> {
        let params = vec![serde_json::json!(_txid), serde_json::json!(_verbose)];
        self.dispatch_json::<GetmempooldescendantsResponse>("getmempooldescendants", &params).await
    }

    /// Returns mempool data for given transaction
    async fn getmempoolentry(
        &self,
        _txid: bitcoin::Txid,
    ) -> Result<GetmempoolentryResponse, TransportError> {
        let params = vec![serde_json::json!(_txid)];
        self.dispatch_json::<GetmempoolentryResponse>("getmempoolentry", &params).await
    }

    /// Returns details on the active state of the TX memory pool.
    async fn getmempoolinfo(&self) -> Result<GetmempoolinfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetmempoolinfoResponse>("getmempoolinfo", &params).await
    }

    /// Returns a json object containing mining-related information.
    async fn getmininginfo(&self) -> Result<GetmininginfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetmininginfoResponse>("getmininginfo", &params).await
    }

    /// Returns information about network traffic, including bytes in, bytes out,
    /// and current system time.
    async fn getnettotals(&self) -> Result<GetnettotalsResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetnettotalsResponse>("getnettotals", &params).await
    }

    /// Returns the estimated network hashes per second based on the last n blocks.
    /// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
    /// Pass in [height] to estimate the network speed at the time when a certain block was found.
    async fn getnetworkhashps(
        &self,
        _nblocks: Option<u64>,
        _height: Option<u64>,
    ) -> Result<GetnetworkhashpsResponse, TransportError> {
        let params = vec![serde_json::json!(_nblocks), serde_json::json!(_height)];
        self.dispatch_json::<GetnetworkhashpsResponse>("getnetworkhashps", &params).await
    }

    /// Returns an object containing various state info regarding P2P networking.
    async fn getnetworkinfo(&self) -> Result<GetnetworkinfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetnetworkinfoResponse>("getnetworkinfo", &params).await
    }

    /// Returns a new Bitcoin address for receiving payments.
    /// If 'label' is specified, it is added to the address book
    /// so payments received with the address will be associated with 'label'.
    async fn getnewaddress(
        &self,
        _label: Option<String>,
        _address_type: Option<String>,
    ) -> Result<GetnewaddressResponse, TransportError> {
        let params = vec![serde_json::json!(_label), serde_json::json!(_address_type)];
        self.dispatch_json::<GetnewaddressResponse>("getnewaddress", &params).await
    }

    /// Return known addresses, after filtering for quality and recency.
    /// These can potentially be used to find new peers in the network.
    /// The total number of addresses known to the node may be higher.
    async fn getnodeaddresses(
        &self,
        _count: Option<u64>,
        _network: Option<String>,
    ) -> Result<GetnodeaddressesResponse, TransportError> {
        let params = vec![serde_json::json!(_count), serde_json::json!(_network)];
        self.dispatch_json::<GetnodeaddressesResponse>("getnodeaddresses", &params).await
    }

    /// Shows transactions in the tx orphanage.
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    async fn getorphantxs(
        &self,
        _verbosity: Option<u32>,
    ) -> Result<GetorphantxsResponse, TransportError> {
        let params = vec![serde_json::json!(_verbosity)];
        self.dispatch_json::<GetorphantxsResponse>("getorphantxs", &params).await
    }

    /// Returns data about each connected network peer as a json array of objects.
    async fn getpeerinfo(&self) -> Result<GetpeerinfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetpeerinfoResponse>("getpeerinfo", &params).await
    }

    /// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
    async fn getprioritisedtransactions(
        &self,
    ) -> Result<GetprioritisedtransactionsResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetprioritisedtransactionsResponse>(
            "getprioritisedtransactions",
            &params,
        )
        .await
    }

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Returns information on all address manager entries for the new and tried tables.
    async fn getrawaddrman(&self) -> Result<GetrawaddrmanResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetrawaddrmanResponse>("getrawaddrman", &params).await
    }

    /// Returns a new Bitcoin address, for receiving change.
    /// This is for use with raw transactions, NOT normal use.
    async fn getrawchangeaddress(
        &self,
        _address_type: Option<String>,
    ) -> Result<GetrawchangeaddressResponse, TransportError> {
        let params = vec![serde_json::json!(_address_type)];
        self.dispatch_json::<GetrawchangeaddressResponse>("getrawchangeaddress", &params).await
    }

    /// Returns all transaction ids in memory pool as a json array of string transaction ids.
    ///
    /// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
    async fn getrawmempool(
        &self,
        _verbose: Option<bool>,
        _mempool_sequence: Option<bool>,
    ) -> Result<GetrawmempoolResponse, TransportError> {
        let params = vec![serde_json::json!(_verbose), serde_json::json!(_mempool_sequence)];
        self.dispatch_json::<GetrawmempoolResponse>("getrawmempool", &params).await
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
    async fn getrawtransaction(
        &self,
        _txid: bitcoin::Txid,
        _verbosity: Option<u32>,
        _blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetrawtransactionResponse, TransportError> {
        let params = vec![
            serde_json::json!(_txid),
            serde_json::json!(_verbosity),
            serde_json::json!(_blockhash),
        ];
        self.dispatch_json::<GetrawtransactionResponse>("getrawtransaction", &params).await
    }

    /// Returns the total amount received by the given address in transactions with at least minconf confirmations.
    async fn getreceivedbyaddress(
        &self,
        _address: String,
        _minconf: Option<u32>,
        _include_immature_coinbase: Option<bool>,
    ) -> Result<GetreceivedbyaddressResponse, TransportError> {
        let params = vec![
            serde_json::json!(_address),
            serde_json::json!(_minconf),
            serde_json::json!(_include_immature_coinbase),
        ];
        self.dispatch_json::<GetreceivedbyaddressResponse>("getreceivedbyaddress", &params).await
    }

    /// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
    async fn getreceivedbylabel(
        &self,
        _label: String,
        _minconf: Option<u32>,
        _include_immature_coinbase: Option<bool>,
    ) -> Result<GetreceivedbylabelResponse, TransportError> {
        let params = vec![
            serde_json::json!(_label),
            serde_json::json!(_minconf),
            serde_json::json!(_include_immature_coinbase),
        ];
        self.dispatch_json::<GetreceivedbylabelResponse>("getreceivedbylabel", &params).await
    }

    /// Returns details of the RPC server.
    async fn getrpcinfo(&self) -> Result<GetrpcinfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetrpcinfoResponse>("getrpcinfo", &params).await
    }

    /// Get detailed information about in-wallet transaction <txid>
    async fn gettransaction(
        &self,
        _txid: bitcoin::Txid,
        _include_watchonly: Option<bool>,
        _verbose: Option<bool>,
    ) -> Result<GettransactionResponse, TransportError> {
        let params = vec![
            serde_json::json!(_txid),
            serde_json::json!(_include_watchonly),
            serde_json::json!(_verbose),
        ];
        self.dispatch_json::<GettransactionResponse>("gettransaction", &params).await
    }

    /// Returns details about an unspent transaction output.
    async fn gettxout(
        &self,
        _txid: bitcoin::Txid,
        _n: u32,
        _include_mempool: Option<bool>,
    ) -> Result<(), TransportError> {
        let params = vec![
            serde_json::json!(_txid),
            serde_json::json!(_n),
            serde_json::json!(_include_mempool),
        ];
        self.dispatch_json::<()>("gettxout", &params).await
    }

    /// Returns a hex-encoded proof that "txid" was included in a block.
    ///
    /// NOTE: By default this function only works sometimes. This is when there is an
    /// unspent output in the utxo for this transaction. To make it always work,
    /// you need to maintain a transaction index, using the -txindex command line option or
    /// specify the block in which the transaction is included manually (by blockhash).
    async fn gettxoutproof(
        &self,
        _txids: Vec<bitcoin::Txid>,
        _blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GettxoutproofResponse, TransportError> {
        let params = vec![serde_json::json!(_txids), serde_json::json!(_blockhash)];
        self.dispatch_json::<GettxoutproofResponse>("gettxoutproof", &params).await
    }

    /// Returns statistics about the unspent transaction output set.
    /// Note this call may take some time if you are not using coinstatsindex.
    async fn gettxoutsetinfo(
        &self,
        _hash_type: Option<String>,
        _hash_or_height: Option<HashOrHeight>,
        _use_index: Option<bool>,
    ) -> Result<GettxoutsetinfoResponse, TransportError> {
        let params = vec![
            serde_json::json!(_hash_type),
            serde_json::json!(_hash_or_height),
            serde_json::json!(_use_index),
        ];
        self.dispatch_json::<GettxoutsetinfoResponse>("gettxoutsetinfo", &params).await
    }

    /// Scans the mempool to find transactions spending any of the given outputs
    async fn gettxspendingprevout(
        &self,
        _outputs: Vec<serde_json::Value>,
    ) -> Result<GettxspendingprevoutResponse, TransportError> {
        let params = vec![serde_json::json!(_outputs)];
        self.dispatch_json::<GettxspendingprevoutResponse>("gettxspendingprevout", &params).await
    }

    /// Returns an object containing various wallet state info.
    async fn getwalletinfo(&self) -> Result<GetwalletinfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetwalletinfoResponse>("getwalletinfo", &params).await
    }

    /// Returns information about the active ZeroMQ notifications.
    async fn getzmqnotifications(&self) -> Result<GetzmqnotificationsResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetzmqnotificationsResponse>("getzmqnotifications", &params).await
    }

    /// List all commands, or get help for a specified command.
    async fn help(&self, _command: Option<String>) -> Result<HelpResponse, TransportError> {
        let params = vec![serde_json::json!(_command)];
        self.dispatch_json::<HelpResponse>("help", &params).await
    }

    /// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
    /// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second element will be imported as an internal descriptor.
    ///
    /// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
    /// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
    /// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
    async fn importdescriptors(
        &self,
        _requests: Vec<serde_json::Value>,
    ) -> Result<ImportdescriptorsResponse, TransportError> {
        let params = vec![serde_json::json!(_requests)];
        self.dispatch_json::<ImportdescriptorsResponse>("importdescriptors", &params).await
    }

    /// Import a mempool.dat file and attempt to add its contents to the mempool.
    /// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
    async fn importmempool(
        &self,
        _filepath: String,
        _options: Option<serde_json::Value>,
    ) -> Result<ImportmempoolResponse, TransportError> {
        let params = vec![serde_json::json!(_filepath), serde_json::json!(_options)];
        self.dispatch_json::<ImportmempoolResponse>("importmempool", &params).await
    }

    /// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
    async fn importprunedfunds(
        &self,
        _rawtransaction: String,
        _txoutproof: String,
    ) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_rawtransaction), serde_json::json!(_txoutproof)];
        self.dispatch_json::<()>("importprunedfunds", &params).await
    }

    /// Permanently marks a block as invalid, as if it violated a consensus rule.
    async fn invalidateblock(&self, _blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_blockhash)];
        self.dispatch_json::<()>("invalidateblock", &params).await
    }

    /// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
    /// No input in any of the PSBTs can be in more than one of the PSBTs.
    async fn joinpsbts(
        &self,
        _txs: Vec<serde_json::Value>,
    ) -> Result<JoinpsbtsResponse, TransportError> {
        let params = vec![serde_json::json!(_txs)];
        self.dispatch_json::<JoinpsbtsResponse>("joinpsbts", &params).await
    }

    /// Refills each descriptor keypool in the wallet up to the specified number of new keys.
    /// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
    ///
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn keypoolrefill(&self, _newsize: Option<u64>) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_newsize)];
        self.dispatch_json::<()>("keypoolrefill", &params).await
    }

    /// Lists groups of addresses which have had their common ownership
    /// made public by common use as inputs or as the resulting change
    /// in past transactions
    async fn listaddressgroupings(&self) -> Result<ListaddressgroupingsResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<ListaddressgroupingsResponse>("listaddressgroupings", &params).await
    }

    /// List all manually banned IPs/Subnets.
    async fn listbanned(&self) -> Result<ListbannedResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<ListbannedResponse>("listbanned", &params).await
    }

    /// List all descriptors present in a wallet.
    async fn listdescriptors(
        &self,
        _private: Option<bool>,
    ) -> Result<ListdescriptorsResponse, TransportError> {
        let params = vec![serde_json::json!(_private)];
        self.dispatch_json::<ListdescriptorsResponse>("listdescriptors", &params).await
    }

    /// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
    async fn listlabels(
        &self,
        _purpose: Option<String>,
    ) -> Result<ListlabelsResponse, TransportError> {
        let params = vec![serde_json::json!(_purpose)];
        self.dispatch_json::<ListlabelsResponse>("listlabels", &params).await
    }

    /// Returns list of temporarily unspendable outputs.
    /// See the lockunspent call to lock and unlock transactions for spending.
    async fn listlockunspent(&self) -> Result<ListlockunspentResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<ListlockunspentResponse>("listlockunspent", &params).await
    }

    /// List balances by receiving address.
    async fn listreceivedbyaddress(
        &self,
        _minconf: Option<u32>,
        _include_empty: Option<bool>,
        _include_watchonly: Option<bool>,
        _address_filter: Option<String>,
        _include_immature_coinbase: Option<bool>,
    ) -> Result<ListreceivedbyaddressResponse, TransportError> {
        let params = vec![
            serde_json::json!(_minconf),
            serde_json::json!(_include_empty),
            serde_json::json!(_include_watchonly),
            serde_json::json!(_address_filter),
            serde_json::json!(_include_immature_coinbase),
        ];
        self.dispatch_json::<ListreceivedbyaddressResponse>("listreceivedbyaddress", &params).await
    }

    /// List received transactions by label.
    async fn listreceivedbylabel(
        &self,
        _minconf: Option<u32>,
        _include_empty: Option<bool>,
        _include_watchonly: Option<bool>,
        _include_immature_coinbase: Option<bool>,
    ) -> Result<ListreceivedbylabelResponse, TransportError> {
        let params = vec![
            serde_json::json!(_minconf),
            serde_json::json!(_include_empty),
            serde_json::json!(_include_watchonly),
            serde_json::json!(_include_immature_coinbase),
        ];
        self.dispatch_json::<ListreceivedbylabelResponse>("listreceivedbylabel", &params).await
    }

    /// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
    /// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
    /// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
    async fn listsinceblock(
        &self,
        _blockhash: Option<bitcoin::BlockHash>,
        _target_confirmations: Option<u64>,
        _include_watchonly: Option<bool>,
        _include_removed: Option<bool>,
        _include_change: Option<bool>,
        _label: Option<String>,
    ) -> Result<ListsinceblockResponse, TransportError> {
        let params = vec![
            serde_json::json!(_blockhash),
            serde_json::json!(_target_confirmations),
            serde_json::json!(_include_watchonly),
            serde_json::json!(_include_removed),
            serde_json::json!(_include_change),
            serde_json::json!(_label),
        ];
        self.dispatch_json::<ListsinceblockResponse>("listsinceblock", &params).await
    }

    /// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
    ///
    /// Returns up to 'count' most recent transactions skipping the first 'from' transactions.
    async fn listtransactions(
        &self,
        _label: Option<String>,
        _count: Option<u64>,
        _skip: Option<u64>,
        _include_watchonly: Option<bool>,
    ) -> Result<ListtransactionsResponse, TransportError> {
        let params = vec![
            serde_json::json!(_label),
            serde_json::json!(_count),
            serde_json::json!(_skip),
            serde_json::json!(_include_watchonly),
        ];
        self.dispatch_json::<ListtransactionsResponse>("listtransactions", &params).await
    }

    /// Returns array of unspent transaction outputs
    /// with between minconf and maxconf (inclusive) confirmations.
    /// Optionally filter to only include txouts paid to specified addresses.
    async fn listunspent(
        &self,
        _minconf: Option<u32>,
        _maxconf: Option<u32>,
        _addresses: Option<Vec<String>>,
        _include_unsafe: Option<bool>,
        _query_options: Option<serde_json::Value>,
    ) -> Result<ListunspentResponse, TransportError> {
        let params = vec![
            serde_json::json!(_minconf),
            serde_json::json!(_maxconf),
            serde_json::json!(_addresses),
            serde_json::json!(_include_unsafe),
            serde_json::json!(_query_options),
        ];
        self.dispatch_json::<ListunspentResponse>("listunspent", &params).await
    }

    /// Returns a list of wallets in the wallet directory.
    async fn listwalletdir(&self) -> Result<ListwalletdirResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<ListwalletdirResponse>("listwalletdir", &params).await
    }

    /// Returns a list of currently loaded wallets.
    /// For full information on the wallet, use "getwalletinfo"
    async fn listwallets(&self) -> Result<ListwalletsResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<ListwalletsResponse>("listwallets", &params).await
    }

    /// Load the serialized UTXO set from a file.
    /// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
    ///
    /// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
    ///
    /// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
    async fn loadtxoutset(&self, _path: String) -> Result<LoadtxoutsetResponse, TransportError> {
        let params = vec![serde_json::json!(_path)];
        self.dispatch_json::<LoadtxoutsetResponse>("loadtxoutset", &params).await
    }

    /// Loads a wallet from a wallet file or directory.
    /// Note that all wallet command-line options used when starting bitcoind will be
    /// applied to the new wallet.
    async fn loadwallet(
        &self,
        _filename: String,
        _load_on_startup: Option<bool>,
    ) -> Result<LoadwalletResponse, TransportError> {
        let params = vec![serde_json::json!(_filename), serde_json::json!(_load_on_startup)];
        self.dispatch_json::<LoadwalletResponse>("loadwallet", &params).await
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
    async fn lockunspent(
        &self,
        _unlock: bool,
        _transactions: Option<Vec<serde_json::Value>>,
        _persistent: Option<bool>,
    ) -> Result<LockunspentResponse, TransportError> {
        let params = vec![
            serde_json::json!(_unlock),
            serde_json::json!(_transactions),
            serde_json::json!(_persistent),
        ];
        self.dispatch_json::<LockunspentResponse>("lockunspent", &params).await
    }

    /// Gets and sets the logging configuration.
    /// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
    /// When called with arguments, adds or removes categories from debug logging and return the lists above.
    /// The arguments are evaluated in order "include", "exclude".
    /// If an item is both included and excluded, it will thus end up being excluded.
    /// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
    /// In addition, the following are available as category names with special meanings:
    /// - "all",  "1" : represent all logging categories.
    async fn logging(
        &self,
        _include: Option<Vec<serde_json::Value>>,
        _exclude: Option<Vec<serde_json::Value>>,
    ) -> Result<LoggingResponse, TransportError> {
        let params = vec![serde_json::json!(_include), serde_json::json!(_exclude)];
        self.dispatch_json::<LoggingResponse>("logging", &params).await
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
    async fn migratewallet(
        &self,
        _wallet_name: Option<String>,
        _passphrase: Option<String>,
    ) -> Result<MigratewalletResponse, TransportError> {
        let params = vec![serde_json::json!(_wallet_name), serde_json::json!(_passphrase)];
        self.dispatch_json::<MigratewalletResponse>("migratewallet", &params).await
    }

    /// Bump the scheduler into the future (-regtest only)
    async fn mockscheduler(&self, _delta_time: u64) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_delta_time)];
        self.dispatch_json::<()>("mockscheduler", &params).await
    }

    /// Requests that a ping be sent to all other nodes, to measure ping time.
    /// Results are provided in getpeerinfo.
    /// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
    async fn ping(&self) -> Result<(), TransportError> {
        let params = vec![];
        self.dispatch_json::<()>("ping", &params).await
    }

    /// Treats a block as if it were received before others with the same work.
    ///
    /// A later preciousblock call can override the effect of an earlier one.
    ///
    /// The effects of preciousblock are not retained across restarts.
    async fn preciousblock(&self, _blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_blockhash)];
        self.dispatch_json::<()>("preciousblock", &params).await
    }

    /// Accepts the transaction into mined blocks at a higher (or lower) priority
    async fn prioritisetransaction(
        &self,
        params: PrioritisetransactionParams,
    ) -> Result<PrioritisetransactionResponse, TransportError> {
        let params = vec![serde_json::json!(params)];
        self.dispatch_json::<PrioritisetransactionResponse>("prioritisetransaction", &params).await
    }

    /// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
    /// Requires `-prune` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via `getblockfrompeer`), local deletion is irreversible.
    async fn pruneblockchain(
        &self,
        _height: u64,
    ) -> Result<PruneblockchainResponse, TransportError> {
        let params = vec![serde_json::json!(_height)];
        self.dispatch_json::<PruneblockchainResponse>("pruneblockchain", &params).await
    }

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
    async fn psbtbumpfee(
        &self,
        _txid: bitcoin::Txid,
        _options: Option<serde_json::Value>,
    ) -> Result<PsbtbumpfeeResponse, TransportError> {
        let params = vec![serde_json::json!(_txid), serde_json::json!(_options)];
        self.dispatch_json::<PsbtbumpfeeResponse>("psbtbumpfee", &params).await
    }

    /// Removes invalidity status of a block, its ancestors and its descendants, reconsider them for activation.
    /// This can be used to undo the effects of invalidateblock.
    async fn reconsiderblock(&self, _blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_blockhash)];
        self.dispatch_json::<()>("reconsiderblock", &params).await
    }

    /// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
    async fn removeprunedfunds(&self, _txid: bitcoin::Txid) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_txid)];
        self.dispatch_json::<()>("removeprunedfunds", &params).await
    }

    /// Rescan the local blockchain for wallet related transactions.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    async fn rescanblockchain(
        &self,
        _start_height: Option<u64>,
        _stop_height: Option<u64>,
    ) -> Result<RescanblockchainResponse, TransportError> {
        let params = vec![serde_json::json!(_start_height), serde_json::json!(_stop_height)];
        self.dispatch_json::<RescanblockchainResponse>("rescanblockchain", &params).await
    }

    /// Restores and loads a wallet from backup.
    ///
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    async fn restorewallet(
        &self,
        _wallet_name: String,
        _backup_file: String,
        _load_on_startup: Option<bool>,
    ) -> Result<RestorewalletResponse, TransportError> {
        let params = vec![
            serde_json::json!(_wallet_name),
            serde_json::json!(_backup_file),
            serde_json::json!(_load_on_startup),
        ];
        self.dispatch_json::<RestorewalletResponse>("restorewallet", &params).await
    }

    /// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
    async fn savemempool(&self) -> Result<SavemempoolResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<SavemempoolResponse>("savemempool", &params).await
    }

    /// Return relevant blockhashes for given descriptors (requires blockfilterindex).
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn scanblocks(
        &self,
        _action: String,
        _scanobjects: Option<Vec<serde_json::Value>>,
        _start_height: Option<u64>,
        _stop_height: Option<u64>,
        _filtertype: Option<String>,
        _options: Option<serde_json::Value>,
    ) -> Result<(), TransportError> {
        let params = vec![
            serde_json::json!(_action),
            serde_json::json!(_scanobjects),
            serde_json::json!(_start_height),
            serde_json::json!(_stop_height),
            serde_json::json!(_filtertype),
            serde_json::json!(_options),
        ];
        self.dispatch_json::<()>("scanblocks", &params).await
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
    /// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
    /// unhardened or hardened child keys.
    /// In the latter case, a range needs to be specified by below if different from 1000.
    /// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
    async fn scantxoutset(
        &self,
        _action: String,
        _scanobjects: Option<Vec<serde_json::Value>>,
    ) -> Result<ScantxoutsetResponse, TransportError> {
        let params = vec![serde_json::json!(_action), serde_json::json!(_scanobjects)];
        self.dispatch_json::<ScantxoutsetResponse>("scantxoutset", &params).await
    }

    /// Return RPC command JSON Schema descriptions.
    async fn schema(&self) -> Result<SchemaResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<SchemaResponse>("schema", &params).await
    }

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Send a transaction.
    async fn send(
        &self,
        _outputs: Vec<serde_json::Value>,
        _conf_target: Option<u64>,
        _estimate_mode: Option<String>,
        _fee_rate: Option<f64>,
        _options: Option<serde_json::Value>,
        _version: Option<u32>,
    ) -> Result<SendResponse, TransportError> {
        let params = vec![
            serde_json::json!(_outputs),
            serde_json::json!(_conf_target),
            serde_json::json!(_estimate_mode),
            serde_json::json!(_fee_rate),
            serde_json::json!(_options),
            serde_json::json!(_version),
        ];
        self.dispatch_json::<SendResponse>("send", &params).await
    }

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
    /// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
    /// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
    async fn sendall(
        &self,
        _recipients: Vec<serde_json::Value>,
        _conf_target: Option<u64>,
        _estimate_mode: Option<String>,
        _fee_rate: Option<f64>,
        _options: Option<serde_json::Value>,
    ) -> Result<SendallResponse, TransportError> {
        let params = vec![
            serde_json::json!(_recipients),
            serde_json::json!(_conf_target),
            serde_json::json!(_estimate_mode),
            serde_json::json!(_fee_rate),
            serde_json::json!(_options),
        ];
        self.dispatch_json::<SendallResponse>("sendall", &params).await
    }

    /// Send multiple times. Amounts are double-precision floating point numbers.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    #[allow(clippy::too_many_arguments)]
    async fn sendmany(&self, params: SendmanyParams) -> Result<SendmanyResponse, TransportError> {
        let params = vec![serde_json::json!(params)];
        self.dispatch_json::<SendmanyResponse>("sendmany", &params).await
    }

    /// Send a p2p message to a peer specified by id.
    /// The message type and body must be provided, the message header will be generated.
    /// This RPC is for testing only.
    async fn sendmsgtopeer(
        &self,
        _peer_id: u64,
        _msg_type: String,
        _msg: String,
    ) -> Result<SendmsgtopeerResponse, TransportError> {
        let params = vec![
            serde_json::json!(_peer_id),
            serde_json::json!(_msg_type),
            serde_json::json!(_msg),
        ];
        self.dispatch_json::<SendmsgtopeerResponse>("sendmsgtopeer", &params).await
    }

    /// Submit a raw transaction (serialized, hex-encoded) to local node and network.
    ///
    /// The transaction will be sent unconditionally to all peers, so using sendrawtransaction
    /// for manual rebroadcast may degrade privacy by leaking the transaction's origin, as
    /// nodes will normally not rebroadcast non-wallet transactions already in their mempool.
    ///
    /// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
    ///
    /// Related RPCs: createrawtransaction, signrawtransactionwithkey
    async fn sendrawtransaction(
        &self,
        _hexstring: String,
        _maxfeerate: Option<f64>,
        _maxburnamount: Option<f64>,
    ) -> Result<SendrawtransactionResponse, TransportError> {
        let params = vec![
            serde_json::json!(_hexstring),
            serde_json::json!(_maxfeerate),
            serde_json::json!(_maxburnamount),
        ];
        self.dispatch_json::<SendrawtransactionResponse>("sendrawtransaction", &params).await
    }

    /// Send an amount to a given address.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    #[allow(clippy::too_many_arguments)]
    async fn sendtoaddress(
        &self,
        _address: String,
        _amount: bitcoin::Amount,
        _comment: Option<String>,
        _comment_to: Option<String>,
        _subtractfeefromamount: Option<bool>,
        _replaceable: Option<bool>,
        _conf_target: Option<u64>,
        _estimate_mode: Option<String>,
        _avoid_reuse: Option<bool>,
        _fee_rate: Option<f64>,
        _verbose: Option<bool>,
    ) -> Result<SendtoaddressResponse, TransportError> {
        let params = vec![
            serde_json::json!(_address),
            serde_json::json!(_amount),
            serde_json::json!(_comment),
            serde_json::json!(_comment_to),
            serde_json::json!(_subtractfeefromamount),
            serde_json::json!(_replaceable),
            serde_json::json!(_conf_target),
            serde_json::json!(_estimate_mode),
            serde_json::json!(_avoid_reuse),
            serde_json::json!(_fee_rate),
            serde_json::json!(_verbose),
        ];
        self.dispatch_json::<SendtoaddressResponse>("sendtoaddress", &params).await
    }

    /// Attempts to add or remove an IP/Subnet from the banned list.
    async fn setban(
        &self,
        _subnet: String,
        _command: String,
        _bantime: Option<u64>,
        _absolute: Option<bool>,
    ) -> Result<(), TransportError> {
        let params = vec![
            serde_json::json!(_subnet),
            serde_json::json!(_command),
            serde_json::json!(_bantime),
            serde_json::json!(_absolute),
        ];
        self.dispatch_json::<()>("setban", &params).await
    }

    /// Sets the label associated with the given address.
    async fn setlabel(&self, _address: String, _label: String) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_address), serde_json::json!(_label)];
        self.dispatch_json::<()>("setlabel", &params).await
    }

    /// Set the local time to given timestamp (-regtest only)
    async fn setmocktime(&self, _timestamp: u64) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_timestamp)];
        self.dispatch_json::<()>("setmocktime", &params).await
    }

    /// Disable/enable all p2p network activity.
    async fn setnetworkactive(
        &self,
        _state: bool,
    ) -> Result<SetnetworkactiveResponse, TransportError> {
        let params = vec![serde_json::json!(_state)];
        self.dispatch_json::<SetnetworkactiveResponse>("setnetworkactive", &params).await
    }

    /// (DEPRECATED) Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
    /// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
    async fn settxfee(&self, _amount: bitcoin::Amount) -> Result<SettxfeeResponse, TransportError> {
        let params = vec![serde_json::json!(_amount)];
        self.dispatch_json::<SettxfeeResponse>("settxfee", &params).await
    }

    /// Change the state of the given wallet flag for a wallet.
    async fn setwalletflag(
        &self,
        _flag: String,
        _value: Option<bool>,
    ) -> Result<SetwalletflagResponse, TransportError> {
        let params = vec![serde_json::json!(_flag), serde_json::json!(_value)];
        self.dispatch_json::<SetwalletflagResponse>("setwalletflag", &params).await
    }

    /// Sign a message with the private key of an address
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn signmessage(
        &self,
        _address: String,
        _message: String,
    ) -> Result<SignmessageResponse, TransportError> {
        let params = vec![serde_json::json!(_address), serde_json::json!(_message)];
        self.dispatch_json::<SignmessageResponse>("signmessage", &params).await
    }

    /// Sign a message with the private key of an address
    async fn signmessagewithprivkey(
        &self,
        _privkey: String,
        _message: String,
    ) -> Result<SignmessagewithprivkeyResponse, TransportError> {
        let params = vec![serde_json::json!(_privkey), serde_json::json!(_message)];
        self.dispatch_json::<SignmessagewithprivkeyResponse>("signmessagewithprivkey", &params)
            .await
    }

    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second argument is an array of base58-encoded private
    /// keys that will be the only keys used to sign the transaction.
    /// The third optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    async fn signrawtransactionwithkey(
        &self,
        _hexstring: String,
        _privkeys: Vec<String>,
        _prevtxs: Option<Vec<serde_json::Value>>,
        _sighashtype: Option<String>,
    ) -> Result<SignrawtransactionwithkeyResponse, TransportError> {
        let params = vec![
            serde_json::json!(_hexstring),
            serde_json::json!(_privkeys),
            serde_json::json!(_prevtxs),
            serde_json::json!(_sighashtype),
        ];
        self.dispatch_json::<SignrawtransactionwithkeyResponse>(
            "signrawtransactionwithkey",
            &params,
        )
        .await
    }

    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn signrawtransactionwithwallet(
        &self,
        _hexstring: String,
        _prevtxs: Option<Vec<serde_json::Value>>,
        _sighashtype: Option<String>,
    ) -> Result<SignrawtransactionwithwalletResponse, TransportError> {
        let params = vec![
            serde_json::json!(_hexstring),
            serde_json::json!(_prevtxs),
            serde_json::json!(_sighashtype),
        ];
        self.dispatch_json::<SignrawtransactionwithwalletResponse>(
            "signrawtransactionwithwallet",
            &params,
        )
        .await
    }

    /// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
    async fn simulaterawtransaction(
        &self,
        _rawtxs: Option<Vec<serde_json::Value>>,
        _options: Option<serde_json::Value>,
    ) -> Result<SimulaterawtransactionResponse, TransportError> {
        let params = vec![serde_json::json!(_rawtxs), serde_json::json!(_options)];
        self.dispatch_json::<SimulaterawtransactionResponse>("simulaterawtransaction", &params)
            .await
    }

    /// Request a graceful shutdown of Bitcoin Core.
    async fn stop(&self, _wait: Option<u64>) -> Result<StopResponse, TransportError> {
        let params = vec![serde_json::json!(_wait)];
        self.dispatch_json::<StopResponse>("stop", &params).await
    }

    /// Attempts to submit new block to network.
    /// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
    async fn submitblock(
        &self,
        _hexdata: String,
        _dummy: Option<String>,
    ) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_hexdata), serde_json::json!(_dummy)];
        self.dispatch_json::<()>("submitblock", &params).await
    }

    /// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
    /// Throws when the header is invalid.
    async fn submitheader(&self, _hexdata: String) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_hexdata)];
        self.dispatch_json::<()>("submitheader", &params).await
    }

    /// Submit a package of raw transactions (serialized, hex-encoded) to local node.
    /// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
    /// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
    /// Warning: successful submission does not mean the transactions will propagate throughout the network.
    async fn submitpackage(
        &self,
        _package: Vec<serde_json::Value>,
        _maxfeerate: Option<f64>,
        _maxburnamount: Option<f64>,
    ) -> Result<SubmitpackageResponse, TransportError> {
        let params = vec![
            serde_json::json!(_package),
            serde_json::json!(_maxfeerate),
            serde_json::json!(_maxburnamount),
        ];
        self.dispatch_json::<SubmitpackageResponse>("submitpackage", &params).await
    }

    /// Waits for the validation interface queue to catch up on everything that was there when we entered this function.
    async fn syncwithvalidationinterfacequeue(&self) -> Result<(), TransportError> {
        let params = vec![];
        self.dispatch_json::<()>("syncwithvalidationinterfacequeue", &params).await
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
    async fn testmempoolaccept(
        &self,
        _rawtxs: Vec<serde_json::Value>,
        _maxfeerate: Option<f64>,
    ) -> Result<TestmempoolacceptResponse, TransportError> {
        let params = vec![serde_json::json!(_rawtxs), serde_json::json!(_maxfeerate)];
        self.dispatch_json::<TestmempoolacceptResponse>("testmempoolaccept", &params).await
    }

    /// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
    /// If both are specified, they must be identical.
    async fn unloadwallet(
        &self,
        _wallet_name: Option<String>,
        _load_on_startup: Option<bool>,
    ) -> Result<UnloadwalletResponse, TransportError> {
        let params = vec![serde_json::json!(_wallet_name), serde_json::json!(_load_on_startup)];
        self.dispatch_json::<UnloadwalletResponse>("unloadwallet", &params).await
    }

    /// Returns the total uptime of the server.
    async fn uptime(&self) -> Result<UptimeResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<UptimeResponse>("uptime", &params).await
    }

    /// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
    async fn utxoupdatepsbt(
        &self,
        _psbt: String,
        _descriptors: Option<Vec<serde_json::Value>>,
    ) -> Result<UtxoupdatepsbtResponse, TransportError> {
        let params = vec![serde_json::json!(_psbt), serde_json::json!(_descriptors)];
        self.dispatch_json::<UtxoupdatepsbtResponse>("utxoupdatepsbt", &params).await
    }

    /// Return information about the given bitcoin address.
    async fn validateaddress(
        &self,
        _address: String,
    ) -> Result<ValidateaddressResponse, TransportError> {
        let params = vec![serde_json::json!(_address)];
        self.dispatch_json::<ValidateaddressResponse>("validateaddress", &params).await
    }

    /// Verifies blockchain database.
    async fn verifychain(
        &self,
        _checklevel: Option<u32>,
        _nblocks: Option<u64>,
    ) -> Result<VerifychainResponse, TransportError> {
        let params = vec![serde_json::json!(_checklevel), serde_json::json!(_nblocks)];
        self.dispatch_json::<VerifychainResponse>("verifychain", &params).await
    }

    /// Verify a signed message.
    async fn verifymessage(
        &self,
        _address: String,
        _signature: String,
        _message: String,
    ) -> Result<VerifymessageResponse, TransportError> {
        let params = vec![
            serde_json::json!(_address),
            serde_json::json!(_signature),
            serde_json::json!(_message),
        ];
        self.dispatch_json::<VerifymessageResponse>("verifymessage", &params).await
    }

    /// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
    /// and throwing an RPC error if the block is not in our best chain
    async fn verifytxoutproof(
        &self,
        _proof: String,
    ) -> Result<VerifytxoutproofResponse, TransportError> {
        let params = vec![serde_json::json!(_proof)];
        self.dispatch_json::<VerifytxoutproofResponse>("verifytxoutproof", &params).await
    }

    /// Waits for a specific new block and returns useful info about it.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn waitforblock(
        &self,
        _blockhash: bitcoin::BlockHash,
        _timeout: Option<u64>,
    ) -> Result<WaitforblockResponse, TransportError> {
        let params = vec![serde_json::json!(_blockhash), serde_json::json!(_timeout)];
        self.dispatch_json::<WaitforblockResponse>("waitforblock", &params).await
    }

    /// Waits for (at least) block height and returns the height and hash
    /// of the current tip.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn waitforblockheight(
        &self,
        _height: u64,
        _timeout: Option<u64>,
    ) -> Result<WaitforblockheightResponse, TransportError> {
        let params = vec![serde_json::json!(_height), serde_json::json!(_timeout)];
        self.dispatch_json::<WaitforblockheightResponse>("waitforblockheight", &params).await
    }

    /// Waits for any new block and returns useful info about it.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn waitfornewblock(
        &self,
        _timeout: Option<u64>,
        _current_tip: Option<String>,
    ) -> Result<WaitfornewblockResponse, TransportError> {
        let params = vec![serde_json::json!(_timeout), serde_json::json!(_current_tip)];
        self.dispatch_json::<WaitfornewblockResponse>("waitfornewblock", &params).await
    }

    /// Creates and funds a transaction in the Partially Signed Transaction format.
    /// Implements the Creator and Updater roles.
    /// All existing inputs must either have their previous output transaction be in the wallet
    /// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
    async fn walletcreatefundedpsbt(
        &self,
        params: WalletcreatefundedpsbtParams,
    ) -> Result<WalletcreatefundedpsbtResponse, TransportError> {
        let params = vec![serde_json::json!(params)];
        self.dispatch_json::<WalletcreatefundedpsbtResponse>("walletcreatefundedpsbt", &params)
            .await
    }

    /// Display address on an external signer for verification.
    async fn walletdisplayaddress(
        &self,
        _address: String,
    ) -> Result<WalletdisplayaddressResponse, TransportError> {
        let params = vec![serde_json::json!(_address)];
        self.dispatch_json::<WalletdisplayaddressResponse>("walletdisplayaddress", &params).await
    }

    /// Removes the wallet encryption key from memory, locking the wallet.
    /// After calling this method, you will need to call walletpassphrase again
    /// before being able to call any methods which require the wallet to be unlocked.
    async fn walletlock(&self) -> Result<(), TransportError> {
        let params = vec![];
        self.dispatch_json::<()>("walletlock", &params).await
    }

    /// Stores the wallet decryption key in memory for 'timeout' seconds.
    /// This is needed prior to performing transactions related to private keys such as sending bitcoins
    ///
    /// Note:
    /// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
    /// time that overrides the old one.
    async fn walletpassphrase(
        &self,
        _passphrase: String,
        _timeout: u64,
    ) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_passphrase), serde_json::json!(_timeout)];
        self.dispatch_json::<()>("walletpassphrase", &params).await
    }

    /// Changes the wallet passphrase from 'oldpassphrase' to 'newpassphrase'.
    async fn walletpassphrasechange(
        &self,
        _oldpassphrase: String,
        _newpassphrase: String,
    ) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_oldpassphrase), serde_json::json!(_newpassphrase)];
        self.dispatch_json::<()>("walletpassphrasechange", &params).await
    }

    /// Update a PSBT with input information from our wallet and then sign inputs
    /// that we can sign for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn walletprocesspsbt(
        &self,
        _psbt: String,
        _sign: Option<bool>,
        _sighashtype: Option<String>,
        _bip32derivs: Option<bool>,
        _finalize: Option<bool>,
    ) -> Result<WalletprocesspsbtResponse, TransportError> {
        let params = vec![
            serde_json::json!(_psbt),
            serde_json::json!(_sign),
            serde_json::json!(_sighashtype),
            serde_json::json!(_bip32derivs),
            serde_json::json!(_finalize),
        ];
        self.dispatch_json::<WalletprocesspsbtResponse>("walletprocesspsbt", &params).await
    }
}

/// Helper to route calls to the node or wallet namespace automatically.
pub trait RpcDispatchExt: TransportTrait + TransportExt {
    /// Dispatch JSON-RPC methods by name.
    fn dispatch_json<R: DeserializeOwned>(
        &self,
        method: &str,
        params: &[serde_json::Value],
    ) -> impl Future<Output = Result<R, TransportError>> + Send {
        async move { self.call(method, params).await }
    }
}

impl<T: TransportTrait + TransportExt + ?Sized> RpcDispatchExt for T {}

// helper trait, so any TransportTrait gets a wallet_call by default
pub trait WalletTransportExt: TransportTrait + TransportExt {
    fn wallet_call<T: serde::Serialize + std::marker::Sync, R: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: &[T],
    ) -> impl std::future::Future<Output = Result<R, crate::transport::TransportError>> + Send {
        async {
            // Convert params to Value before passing to call
            let value_params: Vec<serde_json::Value> =
                params.iter().map(|p| serde_json::to_value(p).unwrap()).collect();
            self.call(method, &value_params).await
        }
    }
}

impl<T: TransportTrait + TransportExt + ?Sized> WalletTransportExt for T {}

// Provide default implementation for any type that implements TransportTrait + TransportExt
#[async_trait]
impl<T: TransportTrait + TransportExt + Send + Sync> BitcoinClientV30 for T {
    /// Mark in-wallet transaction <txid> as abandoned
    /// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
    /// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
    /// It only works on transactions which are not included in a block and are not currently in the mempool.
    /// It has no effect on transactions which are already abandoned.
    async fn abandontransaction(&self, _txid: bitcoin::Txid) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_txid)];
        self.dispatch_json::<()>("abandontransaction", &params).await
    }

    /// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    async fn abortrescan(&self) -> Result<AbortrescanResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<AbortrescanResponse>("abortrescan", &params).await
    }

    /// Open an outbound connection to a specified node. This RPC is for testing only.
    async fn addconnection(
        &self,
        _address: String,
        _connection_type: String,
        _v2transport: bool,
    ) -> Result<AddconnectionResponse, TransportError> {
        let params = vec![
            serde_json::json!(_address),
            serde_json::json!(_connection_type),
            serde_json::json!(_v2transport),
        ];
        self.dispatch_json::<AddconnectionResponse>("addconnection", &params).await
    }

    /// Attempts to add or remove a node from the addnode list.
    /// Or try a connection to a node once.
    /// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
    /// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
    /// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
    async fn addnode(
        &self,
        _node: String,
        _command: String,
        _v2transport: Option<bool>,
    ) -> Result<(), TransportError> {
        let params = vec![
            serde_json::json!(_node),
            serde_json::json!(_command),
            serde_json::json!(_v2transport),
        ];
        self.dispatch_json::<()>("addnode", &params).await
    }

    /// Add the address of a potential peer to an address manager table. This RPC is for testing only.
    async fn addpeeraddress(
        &self,
        _address: String,
        _port: u16,
        _tried: Option<bool>,
    ) -> Result<AddpeeraddressResponse, TransportError> {
        let params =
            vec![serde_json::json!(_address), serde_json::json!(_port), serde_json::json!(_tried)];
        self.dispatch_json::<AddpeeraddressResponse>("addpeeraddress", &params).await
    }

    /// Analyzes and provides information about the current status of a PSBT and its inputs
    async fn analyzepsbt(&self, _psbt: String) -> Result<AnalyzepsbtResponse, TransportError> {
        let params = vec![serde_json::json!(_psbt)];
        self.dispatch_json::<AnalyzepsbtResponse>("analyzepsbt", &params).await
    }

    /// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
    async fn backupwallet(&self, _destination: String) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_destination)];
        self.dispatch_json::<()>("backupwallet", &params).await
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
    async fn bumpfee(
        &self,
        _txid: bitcoin::Txid,
        _options: Option<serde_json::Value>,
    ) -> Result<BumpfeeResponse, TransportError> {
        let params = vec![serde_json::json!(_txid), serde_json::json!(_options)];
        self.dispatch_json::<BumpfeeResponse>("bumpfee", &params).await
    }

    /// Clear all banned IPs.
    async fn clearbanned(&self) -> Result<(), TransportError> {
        let params = vec![];
        self.dispatch_json::<()>("clearbanned", &params).await
    }

    /// Combine multiple partially signed Bitcoin transactions into one transaction.
    /// Implements the Combiner role.
    async fn combinepsbt(
        &self,
        _txs: Vec<serde_json::Value>,
    ) -> Result<CombinepsbtResponse, TransportError> {
        let params = vec![serde_json::json!(_txs)];
        self.dispatch_json::<CombinepsbtResponse>("combinepsbt", &params).await
    }

    /// Combine multiple partially signed transactions into one transaction.
    /// The combined transaction may be another partially signed transaction or a
    /// fully signed transaction.
    async fn combinerawtransaction(
        &self,
        _txs: Vec<serde_json::Value>,
    ) -> Result<CombinerawtransactionResponse, TransportError> {
        let params = vec![serde_json::json!(_txs)];
        self.dispatch_json::<CombinerawtransactionResponse>("combinerawtransaction", &params).await
    }

    /// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
    /// createpsbt and walletcreatefundedpsbt should be used for new applications.
    async fn converttopsbt(
        &self,
        _hexstring: String,
        _permitsigdata: Option<bool>,
        _iswitness: Option<bool>,
    ) -> Result<ConverttopsbtResponse, TransportError> {
        let params = vec![
            serde_json::json!(_hexstring),
            serde_json::json!(_permitsigdata),
            serde_json::json!(_iswitness),
        ];
        self.dispatch_json::<ConverttopsbtResponse>("converttopsbt", &params).await
    }

    /// Creates a multi-signature address with n signatures of m keys required.
    /// It returns a json object with the address and redeemScript.
    async fn createmultisig(
        &self,
        _nrequired: u32,
        _keys: Vec<String>,
        _address_type: Option<String>,
    ) -> Result<CreatemultisigResponse, TransportError> {
        let params = vec![
            serde_json::json!(_nrequired),
            serde_json::json!(_keys),
            serde_json::json!(_address_type),
        ];
        self.dispatch_json::<CreatemultisigResponse>("createmultisig", &params).await
    }

    /// Creates a transaction in the Partially Signed Transaction format.
    /// Implements the Creator role.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    async fn createpsbt(
        &self,
        _inputs: Vec<serde_json::Value>,
        _outputs: Vec<serde_json::Value>,
        _locktime: Option<u32>,
        _replaceable: Option<bool>,
        _version: Option<u32>,
    ) -> Result<CreatepsbtResponse, TransportError> {
        let params = vec![
            serde_json::json!(_inputs),
            serde_json::json!(_outputs),
            serde_json::json!(_locktime),
            serde_json::json!(_replaceable),
            serde_json::json!(_version),
        ];
        self.dispatch_json::<CreatepsbtResponse>("createpsbt", &params).await
    }

    /// Create a transaction spending the given inputs and creating new outputs.
    /// Outputs can be addresses or data.
    /// Returns hex-encoded raw transaction.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    async fn createrawtransaction(
        &self,
        _inputs: Vec<serde_json::Value>,
        _outputs: Vec<serde_json::Value>,
        _locktime: Option<u32>,
        _replaceable: Option<bool>,
        _version: Option<u32>,
    ) -> Result<CreaterawtransactionResponse, TransportError> {
        let params = vec![
            serde_json::json!(_inputs),
            serde_json::json!(_outputs),
            serde_json::json!(_locktime),
            serde_json::json!(_replaceable),
            serde_json::json!(_version),
        ];
        self.dispatch_json::<CreaterawtransactionResponse>("createrawtransaction", &params).await
    }

    /// Creates and loads a new wallet.
    #[allow(clippy::too_many_arguments)]
    async fn createwallet(
        &self,
        _wallet_name: String,
        _disable_private_keys: Option<bool>,
        _blank: Option<bool>,
        _passphrase: Option<String>,
        _avoid_reuse: Option<bool>,
        _descriptors: Option<bool>,
        _load_on_startup: Option<bool>,
        _external_signer: Option<bool>,
    ) -> Result<CreatewalletResponse, TransportError> {
        let params = vec![
            serde_json::json!(_wallet_name),
            serde_json::json!(_disable_private_keys),
            serde_json::json!(_blank),
            serde_json::json!(_passphrase),
            serde_json::json!(_avoid_reuse),
            serde_json::json!(_descriptors),
            serde_json::json!(_load_on_startup),
            serde_json::json!(_external_signer),
        ];
        self.dispatch_json::<CreatewalletResponse>("createwallet", &params).await
    }

    /// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn createwalletdescriptor(
        &self,
        r#_type: String,
        _options: Option<serde_json::Value>,
    ) -> Result<CreatewalletdescriptorResponse, TransportError> {
        let params = vec![serde_json::json!(r#_type), serde_json::json!(_options)];
        self.dispatch_json::<CreatewalletdescriptorResponse>("createwalletdescriptor", &params)
            .await
    }

    /// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
    async fn decodepsbt(&self, _psbt: String) -> Result<DecodepsbtResponse, TransportError> {
        let params = vec![serde_json::json!(_psbt)];
        self.dispatch_json::<DecodepsbtResponse>("decodepsbt", &params).await
    }

    /// Return a JSON object representing the serialized, hex-encoded transaction.
    async fn decoderawtransaction(
        &self,
        _hexstring: String,
        _iswitness: Option<bool>,
    ) -> Result<DecoderawtransactionResponse, TransportError> {
        let params = vec![serde_json::json!(_hexstring), serde_json::json!(_iswitness)];
        self.dispatch_json::<DecoderawtransactionResponse>("decoderawtransaction", &params).await
    }

    /// Decode a hex-encoded script.
    async fn decodescript(
        &self,
        _hexstring: String,
    ) -> Result<DecodescriptResponse, TransportError> {
        let params = vec![serde_json::json!(_hexstring)];
        self.dispatch_json::<DecodescriptResponse>("decodescript", &params).await
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
    async fn deriveaddresses(
        &self,
        _descriptor: String,
        _range: Option<serde_json::Value>,
    ) -> Result<DeriveaddressesResponse, TransportError> {
        let params = vec![serde_json::json!(_descriptor), serde_json::json!(_range)];
        self.dispatch_json::<DeriveaddressesResponse>("deriveaddresses", &params).await
    }

    /// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
    /// Then, sign the inputs we are able to with information from the output descriptors.
    async fn descriptorprocesspsbt(
        &self,
        _psbt: String,
        _descriptors: Vec<serde_json::Value>,
        _sighashtype: Option<String>,
        _bip32derivs: Option<bool>,
        _finalize: Option<bool>,
    ) -> Result<DescriptorprocesspsbtResponse, TransportError> {
        let params = vec![
            serde_json::json!(_psbt),
            serde_json::json!(_descriptors),
            serde_json::json!(_sighashtype),
            serde_json::json!(_bip32derivs),
            serde_json::json!(_finalize),
        ];
        self.dispatch_json::<DescriptorprocesspsbtResponse>("descriptorprocesspsbt", &params).await
    }

    /// Immediately disconnects from the specified peer node.
    ///
    /// Strictly one out of 'address' and 'nodeid' can be provided to identify the node.
    ///
    /// To disconnect by nodeid, either set 'address' to the empty string, or call using the named 'nodeid' argument only.
    async fn disconnectnode(
        &self,
        _address: Option<String>,
        _nodeid: Option<u64>,
    ) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_address), serde_json::json!(_nodeid)];
        self.dispatch_json::<()>("disconnectnode", &params).await
    }

    /// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
    ///
    /// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
    ///
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn dumptxoutset(
        &self,
        _path: String,
        r#_type: Option<String>,
        _options: Option<serde_json::Value>,
    ) -> Result<DumptxoutsetResponse, TransportError> {
        let params =
            vec![serde_json::json!(_path), serde_json::json!(r#_type), serde_json::json!(_options)];
        self.dispatch_json::<DumptxoutsetResponse>("dumptxoutset", &params).await
    }

    /// Simply echo back the input arguments. This command is for testing.
    ///
    /// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
    ///
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    #[allow(clippy::too_many_arguments)]
    async fn echo(
        &self,
        _arg0: Option<String>,
        _arg1: Option<String>,
        _arg2: Option<String>,
        _arg3: Option<String>,
        _arg4: Option<String>,
        _arg5: Option<String>,
        _arg6: Option<String>,
        _arg7: Option<String>,
        _arg8: Option<String>,
        _arg9: Option<String>,
    ) -> Result<EchoResponse, TransportError> {
        let params = vec![
            serde_json::json!(_arg0),
            serde_json::json!(_arg1),
            serde_json::json!(_arg2),
            serde_json::json!(_arg3),
            serde_json::json!(_arg4),
            serde_json::json!(_arg5),
            serde_json::json!(_arg6),
            serde_json::json!(_arg7),
            serde_json::json!(_arg8),
            serde_json::json!(_arg9),
        ];
        self.dispatch_json::<EchoResponse>("echo", &params).await
    }

    /// Echo back the input argument, passing it through a spawned process in a multiprocess build.
    /// This command is for testing.
    async fn echoipc(&self, _arg: String) -> Result<EchoipcResponse, TransportError> {
        let params = vec![serde_json::json!(_arg)];
        self.dispatch_json::<EchoipcResponse>("echoipc", &params).await
    }

    /// Simply echo back the input arguments. This command is for testing.
    ///
    /// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
    ///
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    #[allow(clippy::too_many_arguments)]
    async fn echojson(
        &self,
        _arg0: Option<String>,
        _arg1: Option<String>,
        _arg2: Option<String>,
        _arg3: Option<String>,
        _arg4: Option<String>,
        _arg5: Option<String>,
        _arg6: Option<String>,
        _arg7: Option<String>,
        _arg8: Option<String>,
        _arg9: Option<String>,
    ) -> Result<EchojsonResponse, TransportError> {
        let params = vec![
            serde_json::json!(_arg0),
            serde_json::json!(_arg1),
            serde_json::json!(_arg2),
            serde_json::json!(_arg3),
            serde_json::json!(_arg4),
            serde_json::json!(_arg5),
            serde_json::json!(_arg6),
            serde_json::json!(_arg7),
            serde_json::json!(_arg8),
            serde_json::json!(_arg9),
        ];
        self.dispatch_json::<EchojsonResponse>("echojson", &params).await
    }

    /// Encrypts the wallet with 'passphrase'. This is for first time encryption.
    /// After this, any calls that interact with private keys such as sending or signing
    /// will require the passphrase to be set prior to making these calls.
    /// Use the walletpassphrase call for this, and then walletlock call.
    /// If the wallet is already encrypted, use the walletpassphrasechange call.
    /// ** IMPORTANT **
    /// For security reasons, the encryption process will generate a new HD seed, resulting
    /// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
    /// securely back up the newly generated wallet file using the backupwallet RPC.
    async fn encryptwallet(
        &self,
        _passphrase: String,
    ) -> Result<EncryptwalletResponse, TransportError> {
        let params = vec![serde_json::json!(_passphrase)];
        self.dispatch_json::<EncryptwalletResponse>("encryptwallet", &params).await
    }

    /// Returns a list of external signers from -signer.
    async fn enumeratesigners(&self) -> Result<EnumeratesignersResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<EnumeratesignersResponse>("enumeratesigners", &params).await
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
    async fn estimaterawfee(
        &self,
        _conf_target: u64,
        _threshold: Option<u64>,
    ) -> Result<EstimaterawfeeResponse, TransportError> {
        let params = vec![serde_json::json!(_conf_target), serde_json::json!(_threshold)];
        self.dispatch_json::<EstimaterawfeeResponse>("estimaterawfee", &params).await
    }

    /// Estimates the approximate fee per kilobyte needed for a transaction to begin
    /// confirmation within conf_target blocks if possible and return the number of blocks
    /// for which the estimate is valid. Uses virtual transaction size as defined
    /// in BIP 141 (witness data is discounted).
    async fn estimatesmartfee(
        &self,
        _conf_target: u64,
        _estimate_mode: Option<String>,
    ) -> Result<EstimatesmartfeeResponse, TransportError> {
        let params = vec![serde_json::json!(_conf_target), serde_json::json!(_estimate_mode)];
        self.dispatch_json::<EstimatesmartfeeResponse>("estimatesmartfee", &params).await
    }

    /// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
    /// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
    /// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
    /// Implements the Finalizer and Extractor roles.
    async fn finalizepsbt(
        &self,
        _psbt: String,
        _extract: Option<bool>,
    ) -> Result<FinalizepsbtResponse, TransportError> {
        let params = vec![serde_json::json!(_psbt), serde_json::json!(_extract)];
        self.dispatch_json::<FinalizepsbtResponse>("finalizepsbt", &params).await
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
    async fn fundrawtransaction(
        &self,
        _hexstring: String,
        _options: Option<serde_json::Value>,
        _iswitness: Option<bool>,
    ) -> Result<FundrawtransactionResponse, TransportError> {
        let params = vec![
            serde_json::json!(_hexstring),
            serde_json::json!(_options),
            serde_json::json!(_iswitness),
        ];
        self.dispatch_json::<FundrawtransactionResponse>("fundrawtransaction", &params).await
    }

    /// has been replaced by the -generate cli option. Refer to -help for more information.
    async fn generate(&self) -> Result<(), TransportError> {
        let params = vec![];
        self.dispatch_json::<()>("generate", &params).await
    }

    /// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
    async fn generateblock(
        &self,
        _output: String,
        _transactions: Vec<serde_json::Value>,
        _submit: Option<bool>,
    ) -> Result<GenerateblockResponse, TransportError> {
        let params = vec![
            serde_json::json!(_output),
            serde_json::json!(_transactions),
            serde_json::json!(_submit),
        ];
        self.dispatch_json::<GenerateblockResponse>("generateblock", &params).await
    }

    /// Mine to a specified address and return the block hashes.
    async fn generatetoaddress(
        &self,
        _nblocks: u64,
        _address: String,
        _maxtries: Option<u64>,
    ) -> Result<GeneratetoaddressResponse, TransportError> {
        let params = vec![
            serde_json::json!(_nblocks),
            serde_json::json!(_address),
            serde_json::json!(_maxtries),
        ];
        self.dispatch_json::<GeneratetoaddressResponse>("generatetoaddress", &params).await
    }

    /// Mine to a specified descriptor and return the block hashes.
    async fn generatetodescriptor(
        &self,
        _num_blocks: u64,
        _descriptor: String,
        _maxtries: Option<u64>,
    ) -> Result<GeneratetodescriptorResponse, TransportError> {
        let params = vec![
            serde_json::json!(_num_blocks),
            serde_json::json!(_descriptor),
            serde_json::json!(_maxtries),
        ];
        self.dispatch_json::<GeneratetodescriptorResponse>("generatetodescriptor", &params).await
    }

    /// Returns information about the given added node, or all added nodes
    /// (note that onetry addnodes are not listed here)
    async fn getaddednodeinfo(
        &self,
        _node: Option<String>,
    ) -> Result<GetaddednodeinfoResponse, TransportError> {
        let params = vec![serde_json::json!(_node)];
        self.dispatch_json::<GetaddednodeinfoResponse>("getaddednodeinfo", &params).await
    }

    /// Returns the list of addresses assigned the specified label.
    async fn getaddressesbylabel(
        &self,
        _label: String,
    ) -> Result<GetaddressesbylabelResponse, TransportError> {
        let params = vec![serde_json::json!(_label)];
        self.dispatch_json::<GetaddressesbylabelResponse>("getaddressesbylabel", &params).await
    }

    /// Return information about the given bitcoin address.
    /// Some of the information will only be present if the address is in the active wallet.
    async fn getaddressinfo(
        &self,
        _address: String,
    ) -> Result<GetaddressinfoResponse, TransportError> {
        let params = vec![serde_json::json!(_address)];
        self.dispatch_json::<GetaddressinfoResponse>("getaddressinfo", &params).await
    }

    /// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
    async fn getaddrmaninfo(&self) -> Result<GetaddrmaninfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetaddrmaninfoResponse>("getaddrmaninfo", &params).await
    }

    /// Returns the total available balance.
    /// The available balance is what the wallet considers currently spendable, and is
    /// thus affected by options which limit spendability such as -spendzeroconfchange.
    async fn getbalance(
        &self,
        _dummy: Option<String>,
        _minconf: Option<u32>,
        _include_watchonly: Option<bool>,
        _avoid_reuse: Option<bool>,
    ) -> Result<GetbalanceResponse, TransportError> {
        let params = vec![
            serde_json::json!(_dummy),
            serde_json::json!(_minconf),
            serde_json::json!(_include_watchonly),
            serde_json::json!(_avoid_reuse),
        ];
        self.dispatch_json::<GetbalanceResponse>("getbalance", &params).await
    }

    /// Returns an object with all balances in BTC.
    async fn getbalances(&self) -> Result<GetbalancesResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetbalancesResponse>("getbalances", &params).await
    }

    /// Returns the hash of the best (tip) block in the most-work fully-validated chain.
    async fn getbestblockhash(&self) -> Result<GetbestblockhashResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetbestblockhashResponse>("getbestblockhash", &params).await
    }

    /// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
    /// If verbosity is 1, returns an Object with information about block <hash>.
    /// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
    /// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
    async fn getblock(
        &self,
        _blockhash: bitcoin::BlockHash,
        _verbosity: Option<u32>,
    ) -> Result<GetblockResponse, TransportError> {
        let params = vec![serde_json::json!(_blockhash), serde_json::json!(_verbosity)];
        self.dispatch_json::<GetblockResponse>("getblock", &params).await
    }

    /// Returns an object containing various state info regarding blockchain processing.
    async fn getblockchaininfo(&self) -> Result<GetblockchaininfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetblockchaininfoResponse>("getblockchaininfo", &params).await
    }

    /// Returns the height of the most-work fully-validated chain.
    /// The genesis block has height 0.
    async fn getblockcount(&self) -> Result<GetblockcountResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetblockcountResponse>("getblockcount", &params).await
    }

    /// Retrieve a BIP 157 content filter for a particular block.
    async fn getblockfilter(
        &self,
        _blockhash: bitcoin::BlockHash,
        _filtertype: Option<String>,
    ) -> Result<GetblockfilterResponse, TransportError> {
        let params = vec![serde_json::json!(_blockhash), serde_json::json!(_filtertype)];
        self.dispatch_json::<GetblockfilterResponse>("getblockfilter", &params).await
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
    async fn getblockfrompeer(
        &self,
        _blockhash: bitcoin::BlockHash,
        _peer_id: u64,
    ) -> Result<GetblockfrompeerResponse, TransportError> {
        let params = vec![serde_json::json!(_blockhash), serde_json::json!(_peer_id)];
        self.dispatch_json::<GetblockfrompeerResponse>("getblockfrompeer", &params).await
    }

    /// Returns hash of block in best-block-chain at height provided.
    async fn getblockhash(&self, _height: u64) -> Result<GetblockhashResponse, TransportError> {
        let params = vec![serde_json::json!(_height)];
        self.dispatch_json::<GetblockhashResponse>("getblockhash", &params).await
    }

    /// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
    /// If verbose is true, returns an Object with information about blockheader <hash>.
    async fn getblockheader(
        &self,
        _blockhash: bitcoin::BlockHash,
        _verbose: Option<bool>,
    ) -> Result<GetblockheaderResponse, TransportError> {
        let params = vec![serde_json::json!(_blockhash), serde_json::json!(_verbose)];
        self.dispatch_json::<GetblockheaderResponse>("getblockheader", &params).await
    }

    /// Compute per block statistics for a given window. All amounts are in satoshis.
    /// It won't work for some heights with pruning.
    async fn getblockstats(
        &self,
        _hash_or_height: HashOrHeight,
        _stats: Option<Vec<String>>,
    ) -> Result<GetblockstatsResponse, TransportError> {
        let params = vec![serde_json::json!(_hash_or_height), serde_json::json!(_stats)];
        self.dispatch_json::<GetblockstatsResponse>("getblockstats", &params).await
    }

    /// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
    /// It returns data needed to construct a block to work on.
    /// For full specification, see BIPs 22, 23, 9, and 145:
    /// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
    /// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
    /// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
    /// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
    async fn getblocktemplate(
        &self,
        _template_request: serde_json::Value,
    ) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_template_request)];
        self.dispatch_json::<()>("getblocktemplate", &params).await
    }

    /// Return information about chainstates.
    async fn getchainstates(&self) -> Result<GetchainstatesResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetchainstatesResponse>("getchainstates", &params).await
    }

    /// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
    async fn getchaintips(&self) -> Result<GetchaintipsResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetchaintipsResponse>("getchaintips", &params).await
    }

    /// Compute statistics about the total number and rate of transactions in the chain.
    async fn getchaintxstats(
        &self,
        _nblocks: Option<u64>,
        _blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetchaintxstatsResponse, TransportError> {
        let params = vec![serde_json::json!(_nblocks), serde_json::json!(_blockhash)];
        self.dispatch_json::<GetchaintxstatsResponse>("getchaintxstats", &params).await
    }

    /// Returns the number of connections to other nodes.
    async fn getconnectioncount(&self) -> Result<GetconnectioncountResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetconnectioncountResponse>("getconnectioncount", &params).await
    }

    /// Returns an object containing various state info regarding deployments of consensus changes.
    async fn getdeploymentinfo(
        &self,
        _blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetdeploymentinfoResponse, TransportError> {
        let params = vec![serde_json::json!(_blockhash)];
        self.dispatch_json::<GetdeploymentinfoResponse>("getdeploymentinfo", &params).await
    }

    /// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
    /// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn getdescriptoractivity(
        &self,
        _blockhashes: Vec<serde_json::Value>,
        _scanobjects: Vec<serde_json::Value>,
        _include_mempool: Option<bool>,
    ) -> Result<GetdescriptoractivityResponse, TransportError> {
        let params = vec![
            serde_json::json!(_blockhashes),
            serde_json::json!(_scanobjects),
            serde_json::json!(_include_mempool),
        ];
        self.dispatch_json::<GetdescriptoractivityResponse>("getdescriptoractivity", &params).await
    }

    /// Analyses a descriptor.
    async fn getdescriptorinfo(
        &self,
        _descriptor: String,
    ) -> Result<GetdescriptorinfoResponse, TransportError> {
        let params = vec![serde_json::json!(_descriptor)];
        self.dispatch_json::<GetdescriptorinfoResponse>("getdescriptorinfo", &params).await
    }

    /// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
    async fn getdifficulty(&self) -> Result<GetdifficultyResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetdifficultyResponse>("getdifficulty", &params).await
    }

    /// List all BIP 32 HD keys in the wallet and which descriptors use them.
    async fn gethdkeys(
        &self,
        _options: Option<serde_json::Value>,
    ) -> Result<GethdkeysResponse, TransportError> {
        let params = vec![serde_json::json!(_options)];
        self.dispatch_json::<GethdkeysResponse>("gethdkeys", &params).await
    }

    /// Returns the status of one or all available indices currently running in the node.
    async fn getindexinfo(
        &self,
        _index_name: Option<String>,
    ) -> Result<GetindexinfoResponse, TransportError> {
        let params = vec![serde_json::json!(_index_name)];
        self.dispatch_json::<GetindexinfoResponse>("getindexinfo", &params).await
    }

    /// Returns an object containing information about memory usage.
    async fn getmemoryinfo(
        &self,
        _mode: Option<String>,
    ) -> Result<GetmemoryinfoResponse, TransportError> {
        let params = vec![serde_json::json!(_mode)];
        self.dispatch_json::<GetmemoryinfoResponse>("getmemoryinfo", &params).await
    }

    /// If txid is in the mempool, returns all in-mempool ancestors.
    async fn getmempoolancestors(
        &self,
        _txid: bitcoin::Txid,
        _verbose: Option<bool>,
    ) -> Result<GetmempoolancestorsResponse, TransportError> {
        let params = vec![serde_json::json!(_txid), serde_json::json!(_verbose)];
        self.dispatch_json::<GetmempoolancestorsResponse>("getmempoolancestors", &params).await
    }

    /// If txid is in the mempool, returns all in-mempool descendants.
    async fn getmempooldescendants(
        &self,
        _txid: bitcoin::Txid,
        _verbose: Option<bool>,
    ) -> Result<GetmempooldescendantsResponse, TransportError> {
        let params = vec![serde_json::json!(_txid), serde_json::json!(_verbose)];
        self.dispatch_json::<GetmempooldescendantsResponse>("getmempooldescendants", &params).await
    }

    /// Returns mempool data for given transaction
    async fn getmempoolentry(
        &self,
        _txid: bitcoin::Txid,
    ) -> Result<GetmempoolentryResponse, TransportError> {
        let params = vec![serde_json::json!(_txid)];
        self.dispatch_json::<GetmempoolentryResponse>("getmempoolentry", &params).await
    }

    /// Returns details on the active state of the TX memory pool.
    async fn getmempoolinfo(&self) -> Result<GetmempoolinfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetmempoolinfoResponse>("getmempoolinfo", &params).await
    }

    /// Returns a json object containing mining-related information.
    async fn getmininginfo(&self) -> Result<GetmininginfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetmininginfoResponse>("getmininginfo", &params).await
    }

    /// Returns information about network traffic, including bytes in, bytes out,
    /// and current system time.
    async fn getnettotals(&self) -> Result<GetnettotalsResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetnettotalsResponse>("getnettotals", &params).await
    }

    /// Returns the estimated network hashes per second based on the last n blocks.
    /// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
    /// Pass in [height] to estimate the network speed at the time when a certain block was found.
    async fn getnetworkhashps(
        &self,
        _nblocks: Option<u64>,
        _height: Option<u64>,
    ) -> Result<GetnetworkhashpsResponse, TransportError> {
        let params = vec![serde_json::json!(_nblocks), serde_json::json!(_height)];
        self.dispatch_json::<GetnetworkhashpsResponse>("getnetworkhashps", &params).await
    }

    /// Returns an object containing various state info regarding P2P networking.
    async fn getnetworkinfo(&self) -> Result<GetnetworkinfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetnetworkinfoResponse>("getnetworkinfo", &params).await
    }

    /// Returns a new Bitcoin address for receiving payments.
    /// If 'label' is specified, it is added to the address book
    /// so payments received with the address will be associated with 'label'.
    async fn getnewaddress(
        &self,
        _label: Option<String>,
        _address_type: Option<String>,
    ) -> Result<GetnewaddressResponse, TransportError> {
        let params = vec![serde_json::json!(_label), serde_json::json!(_address_type)];
        self.dispatch_json::<GetnewaddressResponse>("getnewaddress", &params).await
    }

    /// Return known addresses, after filtering for quality and recency.
    /// These can potentially be used to find new peers in the network.
    /// The total number of addresses known to the node may be higher.
    async fn getnodeaddresses(
        &self,
        _count: Option<u64>,
        _network: Option<String>,
    ) -> Result<GetnodeaddressesResponse, TransportError> {
        let params = vec![serde_json::json!(_count), serde_json::json!(_network)];
        self.dispatch_json::<GetnodeaddressesResponse>("getnodeaddresses", &params).await
    }

    /// Shows transactions in the tx orphanage.
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    async fn getorphantxs(
        &self,
        _verbosity: Option<u32>,
    ) -> Result<GetorphantxsResponse, TransportError> {
        let params = vec![serde_json::json!(_verbosity)];
        self.dispatch_json::<GetorphantxsResponse>("getorphantxs", &params).await
    }

    /// Returns data about each connected network peer as a json array of objects.
    async fn getpeerinfo(&self) -> Result<GetpeerinfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetpeerinfoResponse>("getpeerinfo", &params).await
    }

    /// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
    async fn getprioritisedtransactions(
        &self,
    ) -> Result<GetprioritisedtransactionsResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetprioritisedtransactionsResponse>(
            "getprioritisedtransactions",
            &params,
        )
        .await
    }

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Returns information on all address manager entries for the new and tried tables.
    async fn getrawaddrman(&self) -> Result<GetrawaddrmanResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetrawaddrmanResponse>("getrawaddrman", &params).await
    }

    /// Returns a new Bitcoin address, for receiving change.
    /// This is for use with raw transactions, NOT normal use.
    async fn getrawchangeaddress(
        &self,
        _address_type: Option<String>,
    ) -> Result<GetrawchangeaddressResponse, TransportError> {
        let params = vec![serde_json::json!(_address_type)];
        self.dispatch_json::<GetrawchangeaddressResponse>("getrawchangeaddress", &params).await
    }

    /// Returns all transaction ids in memory pool as a json array of string transaction ids.
    ///
    /// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
    async fn getrawmempool(
        &self,
        _verbose: Option<bool>,
        _mempool_sequence: Option<bool>,
    ) -> Result<GetrawmempoolResponse, TransportError> {
        let params = vec![serde_json::json!(_verbose), serde_json::json!(_mempool_sequence)];
        self.dispatch_json::<GetrawmempoolResponse>("getrawmempool", &params).await
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
    async fn getrawtransaction(
        &self,
        _txid: bitcoin::Txid,
        _verbosity: Option<u32>,
        _blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetrawtransactionResponse, TransportError> {
        let params = vec![
            serde_json::json!(_txid),
            serde_json::json!(_verbosity),
            serde_json::json!(_blockhash),
        ];
        self.dispatch_json::<GetrawtransactionResponse>("getrawtransaction", &params).await
    }

    /// Returns the total amount received by the given address in transactions with at least minconf confirmations.
    async fn getreceivedbyaddress(
        &self,
        _address: String,
        _minconf: Option<u32>,
        _include_immature_coinbase: Option<bool>,
    ) -> Result<GetreceivedbyaddressResponse, TransportError> {
        let params = vec![
            serde_json::json!(_address),
            serde_json::json!(_minconf),
            serde_json::json!(_include_immature_coinbase),
        ];
        self.dispatch_json::<GetreceivedbyaddressResponse>("getreceivedbyaddress", &params).await
    }

    /// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
    async fn getreceivedbylabel(
        &self,
        _label: String,
        _minconf: Option<u32>,
        _include_immature_coinbase: Option<bool>,
    ) -> Result<GetreceivedbylabelResponse, TransportError> {
        let params = vec![
            serde_json::json!(_label),
            serde_json::json!(_minconf),
            serde_json::json!(_include_immature_coinbase),
        ];
        self.dispatch_json::<GetreceivedbylabelResponse>("getreceivedbylabel", &params).await
    }

    /// Returns details of the RPC server.
    async fn getrpcinfo(&self) -> Result<GetrpcinfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetrpcinfoResponse>("getrpcinfo", &params).await
    }

    /// Get detailed information about in-wallet transaction <txid>
    async fn gettransaction(
        &self,
        _txid: bitcoin::Txid,
        _include_watchonly: Option<bool>,
        _verbose: Option<bool>,
    ) -> Result<GettransactionResponse, TransportError> {
        let params = vec![
            serde_json::json!(_txid),
            serde_json::json!(_include_watchonly),
            serde_json::json!(_verbose),
        ];
        self.dispatch_json::<GettransactionResponse>("gettransaction", &params).await
    }

    /// Returns details about an unspent transaction output.
    async fn gettxout(
        &self,
        _txid: bitcoin::Txid,
        _n: u32,
        _include_mempool: Option<bool>,
    ) -> Result<(), TransportError> {
        let params = vec![
            serde_json::json!(_txid),
            serde_json::json!(_n),
            serde_json::json!(_include_mempool),
        ];
        self.dispatch_json::<()>("gettxout", &params).await
    }

    /// Returns a hex-encoded proof that "txid" was included in a block.
    ///
    /// NOTE: By default this function only works sometimes. This is when there is an
    /// unspent output in the utxo for this transaction. To make it always work,
    /// you need to maintain a transaction index, using the -txindex command line option or
    /// specify the block in which the transaction is included manually (by blockhash).
    async fn gettxoutproof(
        &self,
        _txids: Vec<bitcoin::Txid>,
        _blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GettxoutproofResponse, TransportError> {
        let params = vec![serde_json::json!(_txids), serde_json::json!(_blockhash)];
        self.dispatch_json::<GettxoutproofResponse>("gettxoutproof", &params).await
    }

    /// Returns statistics about the unspent transaction output set.
    /// Note this call may take some time if you are not using coinstatsindex.
    async fn gettxoutsetinfo(
        &self,
        _hash_type: Option<String>,
        _hash_or_height: Option<HashOrHeight>,
        _use_index: Option<bool>,
    ) -> Result<GettxoutsetinfoResponse, TransportError> {
        let params = vec![
            serde_json::json!(_hash_type),
            serde_json::json!(_hash_or_height),
            serde_json::json!(_use_index),
        ];
        self.dispatch_json::<GettxoutsetinfoResponse>("gettxoutsetinfo", &params).await
    }

    /// Scans the mempool to find transactions spending any of the given outputs
    async fn gettxspendingprevout(
        &self,
        _outputs: Vec<serde_json::Value>,
    ) -> Result<GettxspendingprevoutResponse, TransportError> {
        let params = vec![serde_json::json!(_outputs)];
        self.dispatch_json::<GettxspendingprevoutResponse>("gettxspendingprevout", &params).await
    }

    /// Returns an object containing various wallet state info.
    async fn getwalletinfo(&self) -> Result<GetwalletinfoResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetwalletinfoResponse>("getwalletinfo", &params).await
    }

    /// Returns information about the active ZeroMQ notifications.
    async fn getzmqnotifications(&self) -> Result<GetzmqnotificationsResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<GetzmqnotificationsResponse>("getzmqnotifications", &params).await
    }

    /// List all commands, or get help for a specified command.
    async fn help(&self, _command: Option<String>) -> Result<HelpResponse, TransportError> {
        let params = vec![serde_json::json!(_command)];
        self.dispatch_json::<HelpResponse>("help", &params).await
    }

    /// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
    /// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second element will be imported as an internal descriptor.
    ///
    /// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
    /// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
    /// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
    async fn importdescriptors(
        &self,
        _requests: Vec<serde_json::Value>,
    ) -> Result<ImportdescriptorsResponse, TransportError> {
        let params = vec![serde_json::json!(_requests)];
        self.dispatch_json::<ImportdescriptorsResponse>("importdescriptors", &params).await
    }

    /// Import a mempool.dat file and attempt to add its contents to the mempool.
    /// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
    async fn importmempool(
        &self,
        _filepath: String,
        _options: Option<serde_json::Value>,
    ) -> Result<ImportmempoolResponse, TransportError> {
        let params = vec![serde_json::json!(_filepath), serde_json::json!(_options)];
        self.dispatch_json::<ImportmempoolResponse>("importmempool", &params).await
    }

    /// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
    async fn importprunedfunds(
        &self,
        _rawtransaction: String,
        _txoutproof: String,
    ) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_rawtransaction), serde_json::json!(_txoutproof)];
        self.dispatch_json::<()>("importprunedfunds", &params).await
    }

    /// Permanently marks a block as invalid, as if it violated a consensus rule.
    async fn invalidateblock(&self, _blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_blockhash)];
        self.dispatch_json::<()>("invalidateblock", &params).await
    }

    /// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
    /// No input in any of the PSBTs can be in more than one of the PSBTs.
    async fn joinpsbts(
        &self,
        _txs: Vec<serde_json::Value>,
    ) -> Result<JoinpsbtsResponse, TransportError> {
        let params = vec![serde_json::json!(_txs)];
        self.dispatch_json::<JoinpsbtsResponse>("joinpsbts", &params).await
    }

    /// Refills each descriptor keypool in the wallet up to the specified number of new keys.
    /// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
    ///
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn keypoolrefill(&self, _newsize: Option<u64>) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_newsize)];
        self.dispatch_json::<()>("keypoolrefill", &params).await
    }

    /// Lists groups of addresses which have had their common ownership
    /// made public by common use as inputs or as the resulting change
    /// in past transactions
    async fn listaddressgroupings(&self) -> Result<ListaddressgroupingsResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<ListaddressgroupingsResponse>("listaddressgroupings", &params).await
    }

    /// List all manually banned IPs/Subnets.
    async fn listbanned(&self) -> Result<ListbannedResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<ListbannedResponse>("listbanned", &params).await
    }

    /// List all descriptors present in a wallet.
    async fn listdescriptors(
        &self,
        _private: Option<bool>,
    ) -> Result<ListdescriptorsResponse, TransportError> {
        let params = vec![serde_json::json!(_private)];
        self.dispatch_json::<ListdescriptorsResponse>("listdescriptors", &params).await
    }

    /// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
    async fn listlabels(
        &self,
        _purpose: Option<String>,
    ) -> Result<ListlabelsResponse, TransportError> {
        let params = vec![serde_json::json!(_purpose)];
        self.dispatch_json::<ListlabelsResponse>("listlabels", &params).await
    }

    /// Returns list of temporarily unspendable outputs.
    /// See the lockunspent call to lock and unlock transactions for spending.
    async fn listlockunspent(&self) -> Result<ListlockunspentResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<ListlockunspentResponse>("listlockunspent", &params).await
    }

    /// List balances by receiving address.
    async fn listreceivedbyaddress(
        &self,
        _minconf: Option<u32>,
        _include_empty: Option<bool>,
        _include_watchonly: Option<bool>,
        _address_filter: Option<String>,
        _include_immature_coinbase: Option<bool>,
    ) -> Result<ListreceivedbyaddressResponse, TransportError> {
        let params = vec![
            serde_json::json!(_minconf),
            serde_json::json!(_include_empty),
            serde_json::json!(_include_watchonly),
            serde_json::json!(_address_filter),
            serde_json::json!(_include_immature_coinbase),
        ];
        self.dispatch_json::<ListreceivedbyaddressResponse>("listreceivedbyaddress", &params).await
    }

    /// List received transactions by label.
    async fn listreceivedbylabel(
        &self,
        _minconf: Option<u32>,
        _include_empty: Option<bool>,
        _include_watchonly: Option<bool>,
        _include_immature_coinbase: Option<bool>,
    ) -> Result<ListreceivedbylabelResponse, TransportError> {
        let params = vec![
            serde_json::json!(_minconf),
            serde_json::json!(_include_empty),
            serde_json::json!(_include_watchonly),
            serde_json::json!(_include_immature_coinbase),
        ];
        self.dispatch_json::<ListreceivedbylabelResponse>("listreceivedbylabel", &params).await
    }

    /// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
    /// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
    /// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
    async fn listsinceblock(
        &self,
        _blockhash: Option<bitcoin::BlockHash>,
        _target_confirmations: Option<u64>,
        _include_watchonly: Option<bool>,
        _include_removed: Option<bool>,
        _include_change: Option<bool>,
        _label: Option<String>,
    ) -> Result<ListsinceblockResponse, TransportError> {
        let params = vec![
            serde_json::json!(_blockhash),
            serde_json::json!(_target_confirmations),
            serde_json::json!(_include_watchonly),
            serde_json::json!(_include_removed),
            serde_json::json!(_include_change),
            serde_json::json!(_label),
        ];
        self.dispatch_json::<ListsinceblockResponse>("listsinceblock", &params).await
    }

    /// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
    ///
    /// Returns up to 'count' most recent transactions skipping the first 'from' transactions.
    async fn listtransactions(
        &self,
        _label: Option<String>,
        _count: Option<u64>,
        _skip: Option<u64>,
        _include_watchonly: Option<bool>,
    ) -> Result<ListtransactionsResponse, TransportError> {
        let params = vec![
            serde_json::json!(_label),
            serde_json::json!(_count),
            serde_json::json!(_skip),
            serde_json::json!(_include_watchonly),
        ];
        self.dispatch_json::<ListtransactionsResponse>("listtransactions", &params).await
    }

    /// Returns array of unspent transaction outputs
    /// with between minconf and maxconf (inclusive) confirmations.
    /// Optionally filter to only include txouts paid to specified addresses.
    async fn listunspent(
        &self,
        _minconf: Option<u32>,
        _maxconf: Option<u32>,
        _addresses: Option<Vec<String>>,
        _include_unsafe: Option<bool>,
        _query_options: Option<serde_json::Value>,
    ) -> Result<ListunspentResponse, TransportError> {
        let params = vec![
            serde_json::json!(_minconf),
            serde_json::json!(_maxconf),
            serde_json::json!(_addresses),
            serde_json::json!(_include_unsafe),
            serde_json::json!(_query_options),
        ];
        self.dispatch_json::<ListunspentResponse>("listunspent", &params).await
    }

    /// Returns a list of wallets in the wallet directory.
    async fn listwalletdir(&self) -> Result<ListwalletdirResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<ListwalletdirResponse>("listwalletdir", &params).await
    }

    /// Returns a list of currently loaded wallets.
    /// For full information on the wallet, use "getwalletinfo"
    async fn listwallets(&self) -> Result<ListwalletsResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<ListwalletsResponse>("listwallets", &params).await
    }

    /// Load the serialized UTXO set from a file.
    /// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
    ///
    /// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
    ///
    /// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
    async fn loadtxoutset(&self, _path: String) -> Result<LoadtxoutsetResponse, TransportError> {
        let params = vec![serde_json::json!(_path)];
        self.dispatch_json::<LoadtxoutsetResponse>("loadtxoutset", &params).await
    }

    /// Loads a wallet from a wallet file or directory.
    /// Note that all wallet command-line options used when starting bitcoind will be
    /// applied to the new wallet.
    async fn loadwallet(
        &self,
        _filename: String,
        _load_on_startup: Option<bool>,
    ) -> Result<LoadwalletResponse, TransportError> {
        let params = vec![serde_json::json!(_filename), serde_json::json!(_load_on_startup)];
        self.dispatch_json::<LoadwalletResponse>("loadwallet", &params).await
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
    async fn lockunspent(
        &self,
        _unlock: bool,
        _transactions: Option<Vec<serde_json::Value>>,
        _persistent: Option<bool>,
    ) -> Result<LockunspentResponse, TransportError> {
        let params = vec![
            serde_json::json!(_unlock),
            serde_json::json!(_transactions),
            serde_json::json!(_persistent),
        ];
        self.dispatch_json::<LockunspentResponse>("lockunspent", &params).await
    }

    /// Gets and sets the logging configuration.
    /// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
    /// When called with arguments, adds or removes categories from debug logging and return the lists above.
    /// The arguments are evaluated in order "include", "exclude".
    /// If an item is both included and excluded, it will thus end up being excluded.
    /// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
    /// In addition, the following are available as category names with special meanings:
    /// - "all",  "1" : represent all logging categories.
    async fn logging(
        &self,
        _include: Option<Vec<serde_json::Value>>,
        _exclude: Option<Vec<serde_json::Value>>,
    ) -> Result<LoggingResponse, TransportError> {
        let params = vec![serde_json::json!(_include), serde_json::json!(_exclude)];
        self.dispatch_json::<LoggingResponse>("logging", &params).await
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
    async fn migratewallet(
        &self,
        _wallet_name: Option<String>,
        _passphrase: Option<String>,
    ) -> Result<MigratewalletResponse, TransportError> {
        let params = vec![serde_json::json!(_wallet_name), serde_json::json!(_passphrase)];
        self.dispatch_json::<MigratewalletResponse>("migratewallet", &params).await
    }

    /// Bump the scheduler into the future (-regtest only)
    async fn mockscheduler(&self, _delta_time: u64) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_delta_time)];
        self.dispatch_json::<()>("mockscheduler", &params).await
    }

    /// Requests that a ping be sent to all other nodes, to measure ping time.
    /// Results are provided in getpeerinfo.
    /// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
    async fn ping(&self) -> Result<(), TransportError> {
        let params = vec![];
        self.dispatch_json::<()>("ping", &params).await
    }

    /// Treats a block as if it were received before others with the same work.
    ///
    /// A later preciousblock call can override the effect of an earlier one.
    ///
    /// The effects of preciousblock are not retained across restarts.
    async fn preciousblock(&self, _blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_blockhash)];
        self.dispatch_json::<()>("preciousblock", &params).await
    }

    /// Accepts the transaction into mined blocks at a higher (or lower) priority
    async fn prioritisetransaction(
        &self,
        params: PrioritisetransactionParams,
    ) -> Result<PrioritisetransactionResponse, TransportError> {
        let params = vec![serde_json::json!(params)];
        self.dispatch_json::<PrioritisetransactionResponse>("prioritisetransaction", &params).await
    }

    /// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
    /// Requires `-prune` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via `getblockfrompeer`), local deletion is irreversible.
    async fn pruneblockchain(
        &self,
        _height: u64,
    ) -> Result<PruneblockchainResponse, TransportError> {
        let params = vec![serde_json::json!(_height)];
        self.dispatch_json::<PruneblockchainResponse>("pruneblockchain", &params).await
    }

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
    async fn psbtbumpfee(
        &self,
        _txid: bitcoin::Txid,
        _options: Option<serde_json::Value>,
    ) -> Result<PsbtbumpfeeResponse, TransportError> {
        let params = vec![serde_json::json!(_txid), serde_json::json!(_options)];
        self.dispatch_json::<PsbtbumpfeeResponse>("psbtbumpfee", &params).await
    }

    /// Removes invalidity status of a block, its ancestors and its descendants, reconsider them for activation.
    /// This can be used to undo the effects of invalidateblock.
    async fn reconsiderblock(&self, _blockhash: bitcoin::BlockHash) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_blockhash)];
        self.dispatch_json::<()>("reconsiderblock", &params).await
    }

    /// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
    async fn removeprunedfunds(&self, _txid: bitcoin::Txid) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_txid)];
        self.dispatch_json::<()>("removeprunedfunds", &params).await
    }

    /// Rescan the local blockchain for wallet related transactions.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    async fn rescanblockchain(
        &self,
        _start_height: Option<u64>,
        _stop_height: Option<u64>,
    ) -> Result<RescanblockchainResponse, TransportError> {
        let params = vec![serde_json::json!(_start_height), serde_json::json!(_stop_height)];
        self.dispatch_json::<RescanblockchainResponse>("rescanblockchain", &params).await
    }

    /// Restores and loads a wallet from backup.
    ///
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    async fn restorewallet(
        &self,
        _wallet_name: String,
        _backup_file: String,
        _load_on_startup: Option<bool>,
    ) -> Result<RestorewalletResponse, TransportError> {
        let params = vec![
            serde_json::json!(_wallet_name),
            serde_json::json!(_backup_file),
            serde_json::json!(_load_on_startup),
        ];
        self.dispatch_json::<RestorewalletResponse>("restorewallet", &params).await
    }

    /// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
    async fn savemempool(&self) -> Result<SavemempoolResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<SavemempoolResponse>("savemempool", &params).await
    }

    /// Return relevant blockhashes for given descriptors (requires blockfilterindex).
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn scanblocks(
        &self,
        _action: String,
        _scanobjects: Option<Vec<serde_json::Value>>,
        _start_height: Option<u64>,
        _stop_height: Option<u64>,
        _filtertype: Option<String>,
        _options: Option<serde_json::Value>,
    ) -> Result<(), TransportError> {
        let params = vec![
            serde_json::json!(_action),
            serde_json::json!(_scanobjects),
            serde_json::json!(_start_height),
            serde_json::json!(_stop_height),
            serde_json::json!(_filtertype),
            serde_json::json!(_options),
        ];
        self.dispatch_json::<()>("scanblocks", &params).await
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
    /// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
    /// unhardened or hardened child keys.
    /// In the latter case, a range needs to be specified by below if different from 1000.
    /// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
    async fn scantxoutset(
        &self,
        _action: String,
        _scanobjects: Option<Vec<serde_json::Value>>,
    ) -> Result<ScantxoutsetResponse, TransportError> {
        let params = vec![serde_json::json!(_action), serde_json::json!(_scanobjects)];
        self.dispatch_json::<ScantxoutsetResponse>("scantxoutset", &params).await
    }

    /// Return RPC command JSON Schema descriptions.
    async fn schema(&self) -> Result<SchemaResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<SchemaResponse>("schema", &params).await
    }

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Send a transaction.
    async fn send(
        &self,
        _outputs: Vec<serde_json::Value>,
        _conf_target: Option<u64>,
        _estimate_mode: Option<String>,
        _fee_rate: Option<f64>,
        _options: Option<serde_json::Value>,
        _version: Option<u32>,
    ) -> Result<SendResponse, TransportError> {
        let params = vec![
            serde_json::json!(_outputs),
            serde_json::json!(_conf_target),
            serde_json::json!(_estimate_mode),
            serde_json::json!(_fee_rate),
            serde_json::json!(_options),
            serde_json::json!(_version),
        ];
        self.dispatch_json::<SendResponse>("send", &params).await
    }

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
    /// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
    /// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
    async fn sendall(
        &self,
        _recipients: Vec<serde_json::Value>,
        _conf_target: Option<u64>,
        _estimate_mode: Option<String>,
        _fee_rate: Option<f64>,
        _options: Option<serde_json::Value>,
    ) -> Result<SendallResponse, TransportError> {
        let params = vec![
            serde_json::json!(_recipients),
            serde_json::json!(_conf_target),
            serde_json::json!(_estimate_mode),
            serde_json::json!(_fee_rate),
            serde_json::json!(_options),
        ];
        self.dispatch_json::<SendallResponse>("sendall", &params).await
    }

    /// Send multiple times. Amounts are double-precision floating point numbers.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    #[allow(clippy::too_many_arguments)]
    async fn sendmany(&self, params: SendmanyParams) -> Result<SendmanyResponse, TransportError> {
        let params = vec![serde_json::json!(params)];
        self.dispatch_json::<SendmanyResponse>("sendmany", &params).await
    }

    /// Send a p2p message to a peer specified by id.
    /// The message type and body must be provided, the message header will be generated.
    /// This RPC is for testing only.
    async fn sendmsgtopeer(
        &self,
        _peer_id: u64,
        _msg_type: String,
        _msg: String,
    ) -> Result<SendmsgtopeerResponse, TransportError> {
        let params = vec![
            serde_json::json!(_peer_id),
            serde_json::json!(_msg_type),
            serde_json::json!(_msg),
        ];
        self.dispatch_json::<SendmsgtopeerResponse>("sendmsgtopeer", &params).await
    }

    /// Submit a raw transaction (serialized, hex-encoded) to local node and network.
    ///
    /// The transaction will be sent unconditionally to all peers, so using sendrawtransaction
    /// for manual rebroadcast may degrade privacy by leaking the transaction's origin, as
    /// nodes will normally not rebroadcast non-wallet transactions already in their mempool.
    ///
    /// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
    ///
    /// Related RPCs: createrawtransaction, signrawtransactionwithkey
    async fn sendrawtransaction(
        &self,
        _hexstring: String,
        _maxfeerate: Option<f64>,
        _maxburnamount: Option<f64>,
    ) -> Result<SendrawtransactionResponse, TransportError> {
        let params = vec![
            serde_json::json!(_hexstring),
            serde_json::json!(_maxfeerate),
            serde_json::json!(_maxburnamount),
        ];
        self.dispatch_json::<SendrawtransactionResponse>("sendrawtransaction", &params).await
    }

    /// Send an amount to a given address.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    #[allow(clippy::too_many_arguments)]
    async fn sendtoaddress(
        &self,
        _address: String,
        _amount: bitcoin::Amount,
        _comment: Option<String>,
        _comment_to: Option<String>,
        _subtractfeefromamount: Option<bool>,
        _replaceable: Option<bool>,
        _conf_target: Option<u64>,
        _estimate_mode: Option<String>,
        _avoid_reuse: Option<bool>,
        _fee_rate: Option<f64>,
        _verbose: Option<bool>,
    ) -> Result<SendtoaddressResponse, TransportError> {
        let params = vec![
            serde_json::json!(_address),
            serde_json::json!(_amount),
            serde_json::json!(_comment),
            serde_json::json!(_comment_to),
            serde_json::json!(_subtractfeefromamount),
            serde_json::json!(_replaceable),
            serde_json::json!(_conf_target),
            serde_json::json!(_estimate_mode),
            serde_json::json!(_avoid_reuse),
            serde_json::json!(_fee_rate),
            serde_json::json!(_verbose),
        ];
        self.dispatch_json::<SendtoaddressResponse>("sendtoaddress", &params).await
    }

    /// Attempts to add or remove an IP/Subnet from the banned list.
    async fn setban(
        &self,
        _subnet: String,
        _command: String,
        _bantime: Option<u64>,
        _absolute: Option<bool>,
    ) -> Result<(), TransportError> {
        let params = vec![
            serde_json::json!(_subnet),
            serde_json::json!(_command),
            serde_json::json!(_bantime),
            serde_json::json!(_absolute),
        ];
        self.dispatch_json::<()>("setban", &params).await
    }

    /// Sets the label associated with the given address.
    async fn setlabel(&self, _address: String, _label: String) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_address), serde_json::json!(_label)];
        self.dispatch_json::<()>("setlabel", &params).await
    }

    /// Set the local time to given timestamp (-regtest only)
    async fn setmocktime(&self, _timestamp: u64) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_timestamp)];
        self.dispatch_json::<()>("setmocktime", &params).await
    }

    /// Disable/enable all p2p network activity.
    async fn setnetworkactive(
        &self,
        _state: bool,
    ) -> Result<SetnetworkactiveResponse, TransportError> {
        let params = vec![serde_json::json!(_state)];
        self.dispatch_json::<SetnetworkactiveResponse>("setnetworkactive", &params).await
    }

    /// (DEPRECATED) Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
    /// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
    async fn settxfee(&self, _amount: bitcoin::Amount) -> Result<SettxfeeResponse, TransportError> {
        let params = vec![serde_json::json!(_amount)];
        self.dispatch_json::<SettxfeeResponse>("settxfee", &params).await
    }

    /// Change the state of the given wallet flag for a wallet.
    async fn setwalletflag(
        &self,
        _flag: String,
        _value: Option<bool>,
    ) -> Result<SetwalletflagResponse, TransportError> {
        let params = vec![serde_json::json!(_flag), serde_json::json!(_value)];
        self.dispatch_json::<SetwalletflagResponse>("setwalletflag", &params).await
    }

    /// Sign a message with the private key of an address
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn signmessage(
        &self,
        _address: String,
        _message: String,
    ) -> Result<SignmessageResponse, TransportError> {
        let params = vec![serde_json::json!(_address), serde_json::json!(_message)];
        self.dispatch_json::<SignmessageResponse>("signmessage", &params).await
    }

    /// Sign a message with the private key of an address
    async fn signmessagewithprivkey(
        &self,
        _privkey: String,
        _message: String,
    ) -> Result<SignmessagewithprivkeyResponse, TransportError> {
        let params = vec![serde_json::json!(_privkey), serde_json::json!(_message)];
        self.dispatch_json::<SignmessagewithprivkeyResponse>("signmessagewithprivkey", &params)
            .await
    }

    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second argument is an array of base58-encoded private
    /// keys that will be the only keys used to sign the transaction.
    /// The third optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    async fn signrawtransactionwithkey(
        &self,
        _hexstring: String,
        _privkeys: Vec<String>,
        _prevtxs: Option<Vec<serde_json::Value>>,
        _sighashtype: Option<String>,
    ) -> Result<SignrawtransactionwithkeyResponse, TransportError> {
        let params = vec![
            serde_json::json!(_hexstring),
            serde_json::json!(_privkeys),
            serde_json::json!(_prevtxs),
            serde_json::json!(_sighashtype),
        ];
        self.dispatch_json::<SignrawtransactionwithkeyResponse>(
            "signrawtransactionwithkey",
            &params,
        )
        .await
    }

    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn signrawtransactionwithwallet(
        &self,
        _hexstring: String,
        _prevtxs: Option<Vec<serde_json::Value>>,
        _sighashtype: Option<String>,
    ) -> Result<SignrawtransactionwithwalletResponse, TransportError> {
        let params = vec![
            serde_json::json!(_hexstring),
            serde_json::json!(_prevtxs),
            serde_json::json!(_sighashtype),
        ];
        self.dispatch_json::<SignrawtransactionwithwalletResponse>(
            "signrawtransactionwithwallet",
            &params,
        )
        .await
    }

    /// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
    async fn simulaterawtransaction(
        &self,
        _rawtxs: Option<Vec<serde_json::Value>>,
        _options: Option<serde_json::Value>,
    ) -> Result<SimulaterawtransactionResponse, TransportError> {
        let params = vec![serde_json::json!(_rawtxs), serde_json::json!(_options)];
        self.dispatch_json::<SimulaterawtransactionResponse>("simulaterawtransaction", &params)
            .await
    }

    /// Request a graceful shutdown of Bitcoin Core.
    async fn stop(&self, _wait: Option<u64>) -> Result<StopResponse, TransportError> {
        let params = vec![serde_json::json!(_wait)];
        self.dispatch_json::<StopResponse>("stop", &params).await
    }

    /// Attempts to submit new block to network.
    /// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
    async fn submitblock(
        &self,
        _hexdata: String,
        _dummy: Option<String>,
    ) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_hexdata), serde_json::json!(_dummy)];
        self.dispatch_json::<()>("submitblock", &params).await
    }

    /// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
    /// Throws when the header is invalid.
    async fn submitheader(&self, _hexdata: String) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_hexdata)];
        self.dispatch_json::<()>("submitheader", &params).await
    }

    /// Submit a package of raw transactions (serialized, hex-encoded) to local node.
    /// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
    /// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
    /// Warning: successful submission does not mean the transactions will propagate throughout the network.
    async fn submitpackage(
        &self,
        _package: Vec<serde_json::Value>,
        _maxfeerate: Option<f64>,
        _maxburnamount: Option<f64>,
    ) -> Result<SubmitpackageResponse, TransportError> {
        let params = vec![
            serde_json::json!(_package),
            serde_json::json!(_maxfeerate),
            serde_json::json!(_maxburnamount),
        ];
        self.dispatch_json::<SubmitpackageResponse>("submitpackage", &params).await
    }

    /// Waits for the validation interface queue to catch up on everything that was there when we entered this function.
    async fn syncwithvalidationinterfacequeue(&self) -> Result<(), TransportError> {
        let params = vec![];
        self.dispatch_json::<()>("syncwithvalidationinterfacequeue", &params).await
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
    async fn testmempoolaccept(
        &self,
        _rawtxs: Vec<serde_json::Value>,
        _maxfeerate: Option<f64>,
    ) -> Result<TestmempoolacceptResponse, TransportError> {
        let params = vec![serde_json::json!(_rawtxs), serde_json::json!(_maxfeerate)];
        self.dispatch_json::<TestmempoolacceptResponse>("testmempoolaccept", &params).await
    }

    /// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
    /// If both are specified, they must be identical.
    async fn unloadwallet(
        &self,
        _wallet_name: Option<String>,
        _load_on_startup: Option<bool>,
    ) -> Result<UnloadwalletResponse, TransportError> {
        let params = vec![serde_json::json!(_wallet_name), serde_json::json!(_load_on_startup)];
        self.dispatch_json::<UnloadwalletResponse>("unloadwallet", &params).await
    }

    /// Returns the total uptime of the server.
    async fn uptime(&self) -> Result<UptimeResponse, TransportError> {
        let params = vec![];
        self.dispatch_json::<UptimeResponse>("uptime", &params).await
    }

    /// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
    async fn utxoupdatepsbt(
        &self,
        _psbt: String,
        _descriptors: Option<Vec<serde_json::Value>>,
    ) -> Result<UtxoupdatepsbtResponse, TransportError> {
        let params = vec![serde_json::json!(_psbt), serde_json::json!(_descriptors)];
        self.dispatch_json::<UtxoupdatepsbtResponse>("utxoupdatepsbt", &params).await
    }

    /// Return information about the given bitcoin address.
    async fn validateaddress(
        &self,
        _address: String,
    ) -> Result<ValidateaddressResponse, TransportError> {
        let params = vec![serde_json::json!(_address)];
        self.dispatch_json::<ValidateaddressResponse>("validateaddress", &params).await
    }

    /// Verifies blockchain database.
    async fn verifychain(
        &self,
        _checklevel: Option<u32>,
        _nblocks: Option<u64>,
    ) -> Result<VerifychainResponse, TransportError> {
        let params = vec![serde_json::json!(_checklevel), serde_json::json!(_nblocks)];
        self.dispatch_json::<VerifychainResponse>("verifychain", &params).await
    }

    /// Verify a signed message.
    async fn verifymessage(
        &self,
        _address: String,
        _signature: String,
        _message: String,
    ) -> Result<VerifymessageResponse, TransportError> {
        let params = vec![
            serde_json::json!(_address),
            serde_json::json!(_signature),
            serde_json::json!(_message),
        ];
        self.dispatch_json::<VerifymessageResponse>("verifymessage", &params).await
    }

    /// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
    /// and throwing an RPC error if the block is not in our best chain
    async fn verifytxoutproof(
        &self,
        _proof: String,
    ) -> Result<VerifytxoutproofResponse, TransportError> {
        let params = vec![serde_json::json!(_proof)];
        self.dispatch_json::<VerifytxoutproofResponse>("verifytxoutproof", &params).await
    }

    /// Waits for a specific new block and returns useful info about it.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn waitforblock(
        &self,
        _blockhash: bitcoin::BlockHash,
        _timeout: Option<u64>,
    ) -> Result<WaitforblockResponse, TransportError> {
        let params = vec![serde_json::json!(_blockhash), serde_json::json!(_timeout)];
        self.dispatch_json::<WaitforblockResponse>("waitforblock", &params).await
    }

    /// Waits for (at least) block height and returns the height and hash
    /// of the current tip.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn waitforblockheight(
        &self,
        _height: u64,
        _timeout: Option<u64>,
    ) -> Result<WaitforblockheightResponse, TransportError> {
        let params = vec![serde_json::json!(_height), serde_json::json!(_timeout)];
        self.dispatch_json::<WaitforblockheightResponse>("waitforblockheight", &params).await
    }

    /// Waits for any new block and returns useful info about it.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn waitfornewblock(
        &self,
        _timeout: Option<u64>,
        _current_tip: Option<String>,
    ) -> Result<WaitfornewblockResponse, TransportError> {
        let params = vec![serde_json::json!(_timeout), serde_json::json!(_current_tip)];
        self.dispatch_json::<WaitfornewblockResponse>("waitfornewblock", &params).await
    }

    /// Creates and funds a transaction in the Partially Signed Transaction format.
    /// Implements the Creator and Updater roles.
    /// All existing inputs must either have their previous output transaction be in the wallet
    /// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
    async fn walletcreatefundedpsbt(
        &self,
        params: WalletcreatefundedpsbtParams,
    ) -> Result<WalletcreatefundedpsbtResponse, TransportError> {
        let params = vec![serde_json::json!(params)];
        self.dispatch_json::<WalletcreatefundedpsbtResponse>("walletcreatefundedpsbt", &params)
            .await
    }

    /// Display address on an external signer for verification.
    async fn walletdisplayaddress(
        &self,
        _address: String,
    ) -> Result<WalletdisplayaddressResponse, TransportError> {
        let params = vec![serde_json::json!(_address)];
        self.dispatch_json::<WalletdisplayaddressResponse>("walletdisplayaddress", &params).await
    }

    /// Removes the wallet encryption key from memory, locking the wallet.
    /// After calling this method, you will need to call walletpassphrase again
    /// before being able to call any methods which require the wallet to be unlocked.
    async fn walletlock(&self) -> Result<(), TransportError> {
        let params = vec![];
        self.dispatch_json::<()>("walletlock", &params).await
    }

    /// Stores the wallet decryption key in memory for 'timeout' seconds.
    /// This is needed prior to performing transactions related to private keys such as sending bitcoins
    ///
    /// Note:
    /// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
    /// time that overrides the old one.
    async fn walletpassphrase(
        &self,
        _passphrase: String,
        _timeout: u64,
    ) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_passphrase), serde_json::json!(_timeout)];
        self.dispatch_json::<()>("walletpassphrase", &params).await
    }

    /// Changes the wallet passphrase from 'oldpassphrase' to 'newpassphrase'.
    async fn walletpassphrasechange(
        &self,
        _oldpassphrase: String,
        _newpassphrase: String,
    ) -> Result<(), TransportError> {
        let params = vec![serde_json::json!(_oldpassphrase), serde_json::json!(_newpassphrase)];
        self.dispatch_json::<()>("walletpassphrasechange", &params).await
    }

    /// Update a PSBT with input information from our wallet and then sign inputs
    /// that we can sign for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn walletprocesspsbt(
        &self,
        _psbt: String,
        _sign: Option<bool>,
        _sighashtype: Option<String>,
        _bip32derivs: Option<bool>,
        _finalize: Option<bool>,
    ) -> Result<WalletprocesspsbtResponse, TransportError> {
        let params = vec![
            serde_json::json!(_psbt),
            serde_json::json!(_sign),
            serde_json::json!(_sighashtype),
            serde_json::json!(_bip32derivs),
            serde_json::json!(_finalize),
        ];
        self.dispatch_json::<WalletprocesspsbtResponse>("walletprocesspsbt", &params).await
    }
}
