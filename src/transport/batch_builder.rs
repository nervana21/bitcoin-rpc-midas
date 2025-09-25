use std::sync::Arc;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::responses::*;
use crate::transport::{BatchTransport, TransportError, TransportTrait};
/// Typed results for a JSON-RPC batch
#[derive(Debug, Deserialize)]
pub struct BatchResults {
    pub abandontransaction: (),
    pub abortrescan: Option<AbortrescanResponse>,
    pub addconnection: Option<AddconnectionResponse>,
    pub addnode: (),
    pub addpeeraddress: Option<AddpeeraddressResponse>,
    pub analyzepsbt: Option<AnalyzepsbtResponse>,
    pub backupwallet: (),
    pub bumpfee: Option<BumpfeeResponse>,
    pub clearbanned: (),
    pub combinepsbt: Option<CombinepsbtResponse>,
    pub combinerawtransaction: Option<CombinerawtransactionResponse>,
    pub converttopsbt: Option<ConverttopsbtResponse>,
    pub createmultisig: Option<CreatemultisigResponse>,
    pub createpsbt: Option<CreatepsbtResponse>,
    pub createrawtransaction: Option<CreaterawtransactionResponse>,
    pub createwallet: Option<CreatewalletResponse>,
    pub createwalletdescriptor: Option<CreatewalletdescriptorResponse>,
    pub decodepsbt: Option<DecodepsbtResponse>,
    pub decoderawtransaction: Option<DecoderawtransactionResponse>,
    pub decodescript: Option<DecodescriptResponse>,
    pub deriveaddresses: Option<DeriveaddressesResponse>,
    pub descriptorprocesspsbt: Option<DescriptorprocesspsbtResponse>,
    pub disconnectnode: (),
    pub dumptxoutset: Option<DumptxoutsetResponse>,
    pub echo: Option<EchoResponse>,
    pub echoipc: Option<EchoipcResponse>,
    pub echojson: Option<EchojsonResponse>,
    pub encryptwallet: Option<EncryptwalletResponse>,
    pub enumeratesigners: Option<EnumeratesignersResponse>,
    pub estimaterawfee: Option<EstimaterawfeeResponse>,
    pub estimatesmartfee: Option<EstimatesmartfeeResponse>,
    pub finalizepsbt: Option<FinalizepsbtResponse>,
    pub fundrawtransaction: Option<FundrawtransactionResponse>,
    pub generate: (),
    pub generateblock: Option<GenerateblockResponse>,
    pub generatetoaddress: Option<GeneratetoaddressResponse>,
    pub generatetodescriptor: Option<GeneratetodescriptorResponse>,
    pub getaddednodeinfo: Option<GetaddednodeinfoResponse>,
    pub getaddressesbylabel: Option<GetaddressesbylabelResponse>,
    pub getaddressinfo: Option<GetaddressinfoResponse>,
    pub getaddrmaninfo: Option<GetaddrmaninfoResponse>,
    pub getbalance: Option<GetbalanceResponse>,
    pub getbalances: Option<GetbalancesResponse>,
    pub getbestblockhash: Option<GetbestblockhashResponse>,
    pub getblock: Option<GetblockResponse>,
    pub getblockchaininfo: Option<GetblockchaininfoResponse>,
    pub getblockcount: Option<GetblockcountResponse>,
    pub getblockfilter: Option<GetblockfilterResponse>,
    pub getblockfrompeer: Option<GetblockfrompeerResponse>,
    pub getblockhash: Option<GetblockhashResponse>,
    pub getblockheader: Option<GetblockheaderResponse>,
    pub getblockstats: Option<GetblockstatsResponse>,
    pub getblocktemplate: Option<GetblocktemplateResponse>,
    pub getchainstates: Option<GetchainstatesResponse>,
    pub getchaintips: Option<GetchaintipsResponse>,
    pub getchaintxstats: Option<GetchaintxstatsResponse>,
    pub getconnectioncount: Option<GetconnectioncountResponse>,
    pub getdeploymentinfo: Option<GetdeploymentinfoResponse>,
    pub getdescriptoractivity: Option<GetdescriptoractivityResponse>,
    pub getdescriptorinfo: Option<GetdescriptorinfoResponse>,
    pub getdifficulty: Option<GetdifficultyResponse>,
    pub gethdkeys: Option<GethdkeysResponse>,
    pub getindexinfo: Option<GetindexinfoResponse>,
    pub getmemoryinfo: Option<GetmemoryinfoResponse>,
    pub getmempoolancestors: Option<GetmempoolancestorsResponse>,
    pub getmempooldescendants: Option<GetmempooldescendantsResponse>,
    pub getmempoolentry: Option<GetmempoolentryResponse>,
    pub getmempoolinfo: Option<GetmempoolinfoResponse>,
    pub getmininginfo: Option<GetmininginfoResponse>,
    pub getnettotals: Option<GetnettotalsResponse>,
    pub getnetworkhashps: Option<GetnetworkhashpsResponse>,
    pub getnetworkinfo: Option<GetnetworkinfoResponse>,
    pub getnewaddress: Option<GetnewaddressResponse>,
    pub getnodeaddresses: Option<GetnodeaddressesResponse>,
    pub getorphantxs: Option<GetorphantxsResponse>,
    pub getpeerinfo: Option<GetpeerinfoResponse>,
    pub getprioritisedtransactions: Option<GetprioritisedtransactionsResponse>,
    pub getrawaddrman: Option<GetrawaddrmanResponse>,
    pub getrawchangeaddress: Option<GetrawchangeaddressResponse>,
    pub getrawmempool: Option<GetrawmempoolResponse>,
    pub getrawtransaction: Option<GetrawtransactionResponse>,
    pub getreceivedbyaddress: Option<GetreceivedbyaddressResponse>,
    pub getreceivedbylabel: Option<GetreceivedbylabelResponse>,
    pub getrpcinfo: Option<GetrpcinfoResponse>,
    pub gettransaction: Option<GettransactionResponse>,
    pub gettxout: Option<GettxoutResponse>,
    pub gettxoutproof: Option<GettxoutproofResponse>,
    pub gettxoutsetinfo: Option<GettxoutsetinfoResponse>,
    pub gettxspendingprevout: Option<GettxspendingprevoutResponse>,
    pub getwalletinfo: Option<GetwalletinfoResponse>,
    pub getzmqnotifications: Option<GetzmqnotificationsResponse>,
    pub help: Option<HelpResponse>,
    pub importdescriptors: Option<ImportdescriptorsResponse>,
    pub importmempool: Option<ImportmempoolResponse>,
    pub importprunedfunds: (),
    pub invalidateblock: (),
    pub joinpsbts: Option<JoinpsbtsResponse>,
    pub keypoolrefill: (),
    pub listaddressgroupings: Option<ListaddressgroupingsResponse>,
    pub listbanned: Option<ListbannedResponse>,
    pub listdescriptors: Option<ListdescriptorsResponse>,
    pub listlabels: Option<ListlabelsResponse>,
    pub listlockunspent: Option<ListlockunspentResponse>,
    pub listreceivedbyaddress: Option<ListreceivedbyaddressResponse>,
    pub listreceivedbylabel: Option<ListreceivedbylabelResponse>,
    pub listsinceblock: Option<ListsinceblockResponse>,
    pub listtransactions: Option<ListtransactionsResponse>,
    pub listunspent: Option<ListunspentResponse>,
    pub listwalletdir: Option<ListwalletdirResponse>,
    pub listwallets: Option<ListwalletsResponse>,
    pub loadtxoutset: Option<LoadtxoutsetResponse>,
    pub loadwallet: Option<LoadwalletResponse>,
    pub lockunspent: Option<LockunspentResponse>,
    pub logging: Option<LoggingResponse>,
    pub migratewallet: Option<MigratewalletResponse>,
    pub mockscheduler: (),
    pub ping: (),
    pub preciousblock: (),
    pub prioritisetransaction: Option<PrioritisetransactionResponse>,
    pub pruneblockchain: Option<PruneblockchainResponse>,
    pub psbtbumpfee: Option<PsbtbumpfeeResponse>,
    pub reconsiderblock: (),
    pub removeprunedfunds: (),
    pub rescanblockchain: Option<RescanblockchainResponse>,
    pub restorewallet: Option<RestorewalletResponse>,
    pub savemempool: Option<SavemempoolResponse>,
    pub scanblocks: Option<ScanblocksResponse>,
    pub scantxoutset: Option<ScantxoutsetResponse>,
    pub schema: Option<SchemaResponse>,
    pub send: Option<SendResponse>,
    pub sendall: Option<SendallResponse>,
    pub sendmany: Option<SendmanyResponse>,
    pub sendmsgtopeer: Option<SendmsgtopeerResponse>,
    pub sendrawtransaction: Option<SendrawtransactionResponse>,
    pub sendtoaddress: Option<SendtoaddressResponse>,
    pub setban: (),
    pub setlabel: (),
    pub setmocktime: (),
    pub setnetworkactive: Option<SetnetworkactiveResponse>,
    pub settxfee: Option<SettxfeeResponse>,
    pub setwalletflag: Option<SetwalletflagResponse>,
    pub signmessage: Option<SignmessageResponse>,
    pub signmessagewithprivkey: Option<SignmessagewithprivkeyResponse>,
    pub signrawtransactionwithkey: Option<SignrawtransactionwithkeyResponse>,
    pub signrawtransactionwithwallet: Option<SignrawtransactionwithwalletResponse>,
    pub simulaterawtransaction: Option<SimulaterawtransactionResponse>,
    pub stop: Option<StopResponse>,
    pub submitblock: Option<SubmitblockResponse>,
    pub submitheader: (),
    pub submitpackage: Option<SubmitpackageResponse>,
    pub syncwithvalidationinterfacequeue: (),
    pub testmempoolaccept: Option<TestmempoolacceptResponse>,
    pub unloadwallet: Option<UnloadwalletResponse>,
    pub uptime: Option<UptimeResponse>,
    pub utxoupdatepsbt: Option<UtxoupdatepsbtResponse>,
    pub validateaddress: Option<ValidateaddressResponse>,
    pub verifychain: Option<VerifychainResponse>,
    pub verifymessage: Option<VerifymessageResponse>,
    pub verifytxoutproof: Option<VerifytxoutproofResponse>,
    pub waitforblock: Option<WaitforblockResponse>,
    pub waitforblockheight: Option<WaitforblockheightResponse>,
    pub waitfornewblock: Option<WaitfornewblockResponse>,
    pub walletcreatefundedpsbt: Option<WalletcreatefundedpsbtResponse>,
    pub walletdisplayaddress: Option<WalletdisplayaddressResponse>,
    pub walletlock: (),
    pub walletpassphrase: (),
    pub walletpassphrasechange: (),
    pub walletprocesspsbt: Option<WalletprocesspsbtResponse>,
}

/// Fluent builder for batching multiple RPC calls
pub struct BatchBuilder {
    tx: BatchTransport,
    calls: Vec<(&'static str, Vec<Value>)>,
}

impl BatchBuilder {
    /// Wraps a transport and begins a batch
    pub fn new(inner: Arc<dyn TransportTrait>) -> Self {
        let tx = BatchTransport::new(inner);
        tx.begin_batch();
        BatchBuilder { tx, calls: Vec::new() }
    }

    /// Queue a `abandontransaction` RPC call
    pub fn abandontransaction(mut self, txid: Value) -> Self {
        self.calls.push(("abandontransaction", vec![json!(txid)]));
        self
    }

    /// Queue a `abortrescan` RPC call
    pub fn abortrescan(mut self) -> Self {
        self.calls.push(("abortrescan", Vec::new()));
        self
    }

    /// Queue a `addconnection` RPC call
    pub fn addconnection(
        mut self,
        address: Value,
        connection_type: Value,
        v2transport: Value,
    ) -> Self {
        self.calls.push((
            "addconnection",
            vec![json!(address), json!(connection_type), json!(v2transport)],
        ));
        self
    }

    /// Queue a `addnode` RPC call
    pub fn addnode(mut self, node: Value, command: Value, v2transport: Value) -> Self {
        self.calls.push(("addnode", vec![json!(node), json!(command), json!(v2transport)]));
        self
    }

    /// Queue a `addpeeraddress` RPC call
    pub fn addpeeraddress(mut self, address: Value, port: Value, tried: Value) -> Self {
        self.calls.push(("addpeeraddress", vec![json!(address), json!(port), json!(tried)]));
        self
    }

    /// Queue a `analyzepsbt` RPC call
    pub fn analyzepsbt(mut self, psbt: Value) -> Self {
        self.calls.push(("analyzepsbt", vec![json!(psbt)]));
        self
    }

    /// Queue a `backupwallet` RPC call
    pub fn backupwallet(mut self, destination: Value) -> Self {
        self.calls.push(("backupwallet", vec![json!(destination)]));
        self
    }

    /// Queue a `bumpfee` RPC call
    pub fn bumpfee(mut self, txid: Value, options: Value) -> Self {
        self.calls.push(("bumpfee", vec![json!(txid), json!(options)]));
        self
    }

    /// Queue a `clearbanned` RPC call
    pub fn clearbanned(mut self) -> Self {
        self.calls.push(("clearbanned", Vec::new()));
        self
    }

    /// Queue a `combinepsbt` RPC call
    pub fn combinepsbt(mut self, txs: Value) -> Self {
        self.calls.push(("combinepsbt", vec![json!(txs)]));
        self
    }

    /// Queue a `combinerawtransaction` RPC call
    pub fn combinerawtransaction(mut self, txs: Value) -> Self {
        self.calls.push(("combinerawtransaction", vec![json!(txs)]));
        self
    }

    /// Queue a `converttopsbt` RPC call
    pub fn converttopsbt(
        mut self,
        hexstring: Value,
        permitsigdata: Value,
        iswitness: Value,
    ) -> Self {
        self.calls.push((
            "converttopsbt",
            vec![json!(hexstring), json!(permitsigdata), json!(iswitness)],
        ));
        self
    }

    /// Queue a `createmultisig` RPC call
    pub fn createmultisig(mut self, nrequired: Value, keys: Value, address_type: Value) -> Self {
        self.calls
            .push(("createmultisig", vec![json!(nrequired), json!(keys), json!(address_type)]));
        self
    }

    /// Queue a `createpsbt` RPC call
    pub fn createpsbt(
        mut self,
        inputs: Value,
        outputs: Value,
        locktime: Value,
        replaceable: Value,
        version: Value,
    ) -> Self {
        self.calls.push((
            "createpsbt",
            vec![
                json!(inputs),
                json!(outputs),
                json!(locktime),
                json!(replaceable),
                json!(version),
            ],
        ));
        self
    }

    /// Queue a `createrawtransaction` RPC call
    pub fn createrawtransaction(
        mut self,
        inputs: Value,
        outputs: Value,
        locktime: Value,
        replaceable: Value,
        version: Value,
    ) -> Self {
        self.calls.push((
            "createrawtransaction",
            vec![
                json!(inputs),
                json!(outputs),
                json!(locktime),
                json!(replaceable),
                json!(version),
            ],
        ));
        self
    }

    /// Queue a `createwallet` RPC call
    pub fn createwallet(
        mut self,
        wallet_name: Value,
        disable_private_keys: Value,
        blank: Value,
        passphrase: Value,
        avoid_reuse: Value,
        descriptors: Value,
        load_on_startup: Value,
        external_signer: Value,
    ) -> Self {
        self.calls.push((
            "createwallet",
            vec![
                json!(wallet_name),
                json!(disable_private_keys),
                json!(blank),
                json!(passphrase),
                json!(avoid_reuse),
                json!(descriptors),
                json!(load_on_startup),
                json!(external_signer),
            ],
        ));
        self
    }

    /// Queue a `createwalletdescriptor` RPC call
    pub fn createwalletdescriptor(mut self, r#type: Value, options: Value) -> Self {
        self.calls.push(("createwalletdescriptor", vec![json!(r#type), json!(options)]));
        self
    }

    /// Queue a `decodepsbt` RPC call
    pub fn decodepsbt(mut self, psbt: Value) -> Self {
        self.calls.push(("decodepsbt", vec![json!(psbt)]));
        self
    }

    /// Queue a `decoderawtransaction` RPC call
    pub fn decoderawtransaction(mut self, hexstring: Value, iswitness: Value) -> Self {
        self.calls.push(("decoderawtransaction", vec![json!(hexstring), json!(iswitness)]));
        self
    }

    /// Queue a `decodescript` RPC call
    pub fn decodescript(mut self, hexstring: Value) -> Self {
        self.calls.push(("decodescript", vec![json!(hexstring)]));
        self
    }

    /// Queue a `deriveaddresses` RPC call
    pub fn deriveaddresses(mut self, descriptor: Value, range: Value) -> Self {
        self.calls.push(("deriveaddresses", vec![json!(descriptor), json!(range)]));
        self
    }

    /// Queue a `descriptorprocesspsbt` RPC call
    pub fn descriptorprocesspsbt(
        mut self,
        psbt: Value,
        descriptors: Value,
        sighashtype: Value,
        bip32derivs: Value,
        finalize: Value,
    ) -> Self {
        self.calls.push((
            "descriptorprocesspsbt",
            vec![
                json!(psbt),
                json!(descriptors),
                json!(sighashtype),
                json!(bip32derivs),
                json!(finalize),
            ],
        ));
        self
    }

    /// Queue a `disconnectnode` RPC call
    pub fn disconnectnode(mut self, address: Value, nodeid: Value) -> Self {
        self.calls.push(("disconnectnode", vec![json!(address), json!(nodeid)]));
        self
    }

    /// Queue a `dumptxoutset` RPC call
    pub fn dumptxoutset(mut self, path: Value, r#type: Value, options: Value) -> Self {
        self.calls.push(("dumptxoutset", vec![json!(path), json!(r#type), json!(options)]));
        self
    }

    /// Queue a `echo` RPC call
    pub fn echo(
        mut self,
        arg0: Value,
        arg1: Value,
        arg2: Value,
        arg3: Value,
        arg4: Value,
        arg5: Value,
        arg6: Value,
        arg7: Value,
        arg8: Value,
        arg9: Value,
    ) -> Self {
        self.calls.push((
            "echo",
            vec![
                json!(arg0),
                json!(arg1),
                json!(arg2),
                json!(arg3),
                json!(arg4),
                json!(arg5),
                json!(arg6),
                json!(arg7),
                json!(arg8),
                json!(arg9),
            ],
        ));
        self
    }

    /// Queue a `echoipc` RPC call
    pub fn echoipc(mut self, arg: Value) -> Self {
        self.calls.push(("echoipc", vec![json!(arg)]));
        self
    }

    /// Queue a `echojson` RPC call
    pub fn echojson(
        mut self,
        arg0: Value,
        arg1: Value,
        arg2: Value,
        arg3: Value,
        arg4: Value,
        arg5: Value,
        arg6: Value,
        arg7: Value,
        arg8: Value,
        arg9: Value,
    ) -> Self {
        self.calls.push((
            "echojson",
            vec![
                json!(arg0),
                json!(arg1),
                json!(arg2),
                json!(arg3),
                json!(arg4),
                json!(arg5),
                json!(arg6),
                json!(arg7),
                json!(arg8),
                json!(arg9),
            ],
        ));
        self
    }

    /// Queue a `encryptwallet` RPC call
    pub fn encryptwallet(mut self, passphrase: Value) -> Self {
        self.calls.push(("encryptwallet", vec![json!(passphrase)]));
        self
    }

    /// Queue a `enumeratesigners` RPC call
    pub fn enumeratesigners(mut self) -> Self {
        self.calls.push(("enumeratesigners", Vec::new()));
        self
    }

    /// Queue a `estimaterawfee` RPC call
    pub fn estimaterawfee(mut self, conf_target: Value, threshold: Value) -> Self {
        self.calls.push(("estimaterawfee", vec![json!(conf_target), json!(threshold)]));
        self
    }

    /// Queue a `estimatesmartfee` RPC call
    pub fn estimatesmartfee(mut self, conf_target: Value, estimate_mode: Value) -> Self {
        self.calls.push(("estimatesmartfee", vec![json!(conf_target), json!(estimate_mode)]));
        self
    }

    /// Queue a `finalizepsbt` RPC call
    pub fn finalizepsbt(mut self, psbt: Value, extract: Value) -> Self {
        self.calls.push(("finalizepsbt", vec![json!(psbt), json!(extract)]));
        self
    }

    /// Queue a `fundrawtransaction` RPC call
    pub fn fundrawtransaction(
        mut self,
        hexstring: Value,
        options: Value,
        iswitness: Value,
    ) -> Self {
        self.calls
            .push(("fundrawtransaction", vec![json!(hexstring), json!(options), json!(iswitness)]));
        self
    }

    /// Queue a `generate` RPC call
    pub fn generate(mut self) -> Self {
        self.calls.push(("generate", Vec::new()));
        self
    }

    /// Queue a `generateblock` RPC call
    pub fn generateblock(mut self, output: Value, transactions: Value, submit: Value) -> Self {
        self.calls.push(("generateblock", vec![json!(output), json!(transactions), json!(submit)]));
        self
    }

    /// Queue a `generatetoaddress` RPC call
    pub fn generatetoaddress(mut self, nblocks: Value, address: Value, maxtries: Value) -> Self {
        self.calls
            .push(("generatetoaddress", vec![json!(nblocks), json!(address), json!(maxtries)]));
        self
    }

    /// Queue a `generatetodescriptor` RPC call
    pub fn generatetodescriptor(
        mut self,
        num_blocks: Value,
        descriptor: Value,
        maxtries: Value,
    ) -> Self {
        self.calls.push((
            "generatetodescriptor",
            vec![json!(num_blocks), json!(descriptor), json!(maxtries)],
        ));
        self
    }

    /// Queue a `getaddednodeinfo` RPC call
    pub fn getaddednodeinfo(mut self, node: Value) -> Self {
        self.calls.push(("getaddednodeinfo", vec![json!(node)]));
        self
    }

    /// Queue a `getaddressesbylabel` RPC call
    pub fn getaddressesbylabel(mut self, label: Value) -> Self {
        self.calls.push(("getaddressesbylabel", vec![json!(label)]));
        self
    }

    /// Queue a `getaddressinfo` RPC call
    pub fn getaddressinfo(mut self, address: Value) -> Self {
        self.calls.push(("getaddressinfo", vec![json!(address)]));
        self
    }

    /// Queue a `getaddrmaninfo` RPC call
    pub fn getaddrmaninfo(mut self) -> Self {
        self.calls.push(("getaddrmaninfo", Vec::new()));
        self
    }

    /// Queue a `getbalance` RPC call
    pub fn getbalance(
        mut self,
        dummy: Value,
        minconf: Value,
        include_watchonly: Value,
        avoid_reuse: Value,
    ) -> Self {
        self.calls.push((
            "getbalance",
            vec![json!(dummy), json!(minconf), json!(include_watchonly), json!(avoid_reuse)],
        ));
        self
    }

    /// Queue a `getbalances` RPC call
    pub fn getbalances(mut self) -> Self {
        self.calls.push(("getbalances", Vec::new()));
        self
    }

    /// Queue a `getbestblockhash` RPC call
    pub fn getbestblockhash(mut self) -> Self {
        self.calls.push(("getbestblockhash", Vec::new()));
        self
    }

    /// Queue a `getblock` RPC call
    pub fn getblock(mut self, blockhash: Value, verbosity: Value) -> Self {
        self.calls.push(("getblock", vec![json!(blockhash), json!(verbosity)]));
        self
    }

    /// Queue a `getblockchaininfo` RPC call
    pub fn getblockchaininfo(mut self) -> Self {
        self.calls.push(("getblockchaininfo", Vec::new()));
        self
    }

    /// Queue a `getblockcount` RPC call
    pub fn getblockcount(mut self) -> Self {
        self.calls.push(("getblockcount", Vec::new()));
        self
    }

    /// Queue a `getblockfilter` RPC call
    pub fn getblockfilter(mut self, blockhash: Value, filtertype: Value) -> Self {
        self.calls.push(("getblockfilter", vec![json!(blockhash), json!(filtertype)]));
        self
    }

    /// Queue a `getblockfrompeer` RPC call
    pub fn getblockfrompeer(mut self, blockhash: Value, peer_id: Value) -> Self {
        self.calls.push(("getblockfrompeer", vec![json!(blockhash), json!(peer_id)]));
        self
    }

    /// Queue a `getblockhash` RPC call
    pub fn getblockhash(mut self, height: Value) -> Self {
        self.calls.push(("getblockhash", vec![json!(height)]));
        self
    }

    /// Queue a `getblockheader` RPC call
    pub fn getblockheader(mut self, blockhash: Value, verbose: Value) -> Self {
        self.calls.push(("getblockheader", vec![json!(blockhash), json!(verbose)]));
        self
    }

    /// Queue a `getblockstats` RPC call
    pub fn getblockstats(mut self, hash_or_height: Value, stats: Value) -> Self {
        self.calls.push(("getblockstats", vec![json!(hash_or_height), json!(stats)]));
        self
    }

    /// Queue a `getblocktemplate` RPC call
    pub fn getblocktemplate(mut self, template_request: Value) -> Self {
        self.calls.push(("getblocktemplate", vec![json!(template_request)]));
        self
    }

    /// Queue a `getchainstates` RPC call
    pub fn getchainstates(mut self) -> Self {
        self.calls.push(("getchainstates", Vec::new()));
        self
    }

    /// Queue a `getchaintips` RPC call
    pub fn getchaintips(mut self) -> Self {
        self.calls.push(("getchaintips", Vec::new()));
        self
    }

    /// Queue a `getchaintxstats` RPC call
    pub fn getchaintxstats(mut self, nblocks: Value, blockhash: Value) -> Self {
        self.calls.push(("getchaintxstats", vec![json!(nblocks), json!(blockhash)]));
        self
    }

    /// Queue a `getconnectioncount` RPC call
    pub fn getconnectioncount(mut self) -> Self {
        self.calls.push(("getconnectioncount", Vec::new()));
        self
    }

    /// Queue a `getdeploymentinfo` RPC call
    pub fn getdeploymentinfo(mut self, blockhash: Value) -> Self {
        self.calls.push(("getdeploymentinfo", vec![json!(blockhash)]));
        self
    }

    /// Queue a `getdescriptoractivity` RPC call
    pub fn getdescriptoractivity(
        mut self,
        blockhashes: Value,
        scanobjects: Value,
        include_mempool: Value,
    ) -> Self {
        self.calls.push((
            "getdescriptoractivity",
            vec![json!(blockhashes), json!(scanobjects), json!(include_mempool)],
        ));
        self
    }

    /// Queue a `getdescriptorinfo` RPC call
    pub fn getdescriptorinfo(mut self, descriptor: Value) -> Self {
        self.calls.push(("getdescriptorinfo", vec![json!(descriptor)]));
        self
    }

    /// Queue a `getdifficulty` RPC call
    pub fn getdifficulty(mut self) -> Self {
        self.calls.push(("getdifficulty", Vec::new()));
        self
    }

    /// Queue a `gethdkeys` RPC call
    pub fn gethdkeys(mut self, options: Value) -> Self {
        self.calls.push(("gethdkeys", vec![json!(options)]));
        self
    }

    /// Queue a `getindexinfo` RPC call
    pub fn getindexinfo(mut self, index_name: Value) -> Self {
        self.calls.push(("getindexinfo", vec![json!(index_name)]));
        self
    }

    /// Queue a `getmemoryinfo` RPC call
    pub fn getmemoryinfo(mut self, mode: Value) -> Self {
        self.calls.push(("getmemoryinfo", vec![json!(mode)]));
        self
    }

    /// Queue a `getmempoolancestors` RPC call
    pub fn getmempoolancestors(mut self, txid: Value, verbose: Value) -> Self {
        self.calls.push(("getmempoolancestors", vec![json!(txid), json!(verbose)]));
        self
    }

    /// Queue a `getmempooldescendants` RPC call
    pub fn getmempooldescendants(mut self, txid: Value, verbose: Value) -> Self {
        self.calls.push(("getmempooldescendants", vec![json!(txid), json!(verbose)]));
        self
    }

    /// Queue a `getmempoolentry` RPC call
    pub fn getmempoolentry(mut self, txid: Value) -> Self {
        self.calls.push(("getmempoolentry", vec![json!(txid)]));
        self
    }

    /// Queue a `getmempoolinfo` RPC call
    pub fn getmempoolinfo(mut self) -> Self {
        self.calls.push(("getmempoolinfo", Vec::new()));
        self
    }

    /// Queue a `getmininginfo` RPC call
    pub fn getmininginfo(mut self) -> Self {
        self.calls.push(("getmininginfo", Vec::new()));
        self
    }

    /// Queue a `getnettotals` RPC call
    pub fn getnettotals(mut self) -> Self {
        self.calls.push(("getnettotals", Vec::new()));
        self
    }

    /// Queue a `getnetworkhashps` RPC call
    pub fn getnetworkhashps(mut self, nblocks: Value, height: Value) -> Self {
        self.calls.push(("getnetworkhashps", vec![json!(nblocks), json!(height)]));
        self
    }

    /// Queue a `getnetworkinfo` RPC call
    pub fn getnetworkinfo(mut self) -> Self {
        self.calls.push(("getnetworkinfo", Vec::new()));
        self
    }

    /// Queue a `getnewaddress` RPC call
    pub fn getnewaddress(mut self, label: Value, address_type: Value) -> Self {
        self.calls.push(("getnewaddress", vec![json!(label), json!(address_type)]));
        self
    }

    /// Queue a `getnodeaddresses` RPC call
    pub fn getnodeaddresses(mut self, count: Value, network: Value) -> Self {
        self.calls.push(("getnodeaddresses", vec![json!(count), json!(network)]));
        self
    }

    /// Queue a `getorphantxs` RPC call
    pub fn getorphantxs(mut self, verbosity: Value) -> Self {
        self.calls.push(("getorphantxs", vec![json!(verbosity)]));
        self
    }

    /// Queue a `getpeerinfo` RPC call
    pub fn getpeerinfo(mut self) -> Self {
        self.calls.push(("getpeerinfo", Vec::new()));
        self
    }

    /// Queue a `getprioritisedtransactions` RPC call
    pub fn getprioritisedtransactions(mut self) -> Self {
        self.calls.push(("getprioritisedtransactions", Vec::new()));
        self
    }

    /// Queue a `getrawaddrman` RPC call
    pub fn getrawaddrman(mut self) -> Self {
        self.calls.push(("getrawaddrman", Vec::new()));
        self
    }

    /// Queue a `getrawchangeaddress` RPC call
    pub fn getrawchangeaddress(mut self, address_type: Value) -> Self {
        self.calls.push(("getrawchangeaddress", vec![json!(address_type)]));
        self
    }

    /// Queue a `getrawmempool` RPC call
    pub fn getrawmempool(mut self, verbose: Value, mempool_sequence: Value) -> Self {
        self.calls.push(("getrawmempool", vec![json!(verbose), json!(mempool_sequence)]));
        self
    }

    /// Queue a `getrawtransaction` RPC call
    pub fn getrawtransaction(mut self, txid: Value, verbosity: Value, blockhash: Value) -> Self {
        self.calls
            .push(("getrawtransaction", vec![json!(txid), json!(verbosity), json!(blockhash)]));
        self
    }

    /// Queue a `getreceivedbyaddress` RPC call
    pub fn getreceivedbyaddress(
        mut self,
        address: Value,
        minconf: Value,
        include_immature_coinbase: Value,
    ) -> Self {
        self.calls.push((
            "getreceivedbyaddress",
            vec![json!(address), json!(minconf), json!(include_immature_coinbase)],
        ));
        self
    }

    /// Queue a `getreceivedbylabel` RPC call
    pub fn getreceivedbylabel(
        mut self,
        label: Value,
        minconf: Value,
        include_immature_coinbase: Value,
    ) -> Self {
        self.calls.push((
            "getreceivedbylabel",
            vec![json!(label), json!(minconf), json!(include_immature_coinbase)],
        ));
        self
    }

    /// Queue a `getrpcinfo` RPC call
    pub fn getrpcinfo(mut self) -> Self {
        self.calls.push(("getrpcinfo", Vec::new()));
        self
    }

    /// Queue a `gettransaction` RPC call
    pub fn gettransaction(mut self, txid: Value, include_watchonly: Value, verbose: Value) -> Self {
        self.calls
            .push(("gettransaction", vec![json!(txid), json!(include_watchonly), json!(verbose)]));
        self
    }

    /// Queue a `gettxout` RPC call
    pub fn gettxout(mut self, txid: Value, n: Value, include_mempool: Value) -> Self {
        self.calls.push(("gettxout", vec![json!(txid), json!(n), json!(include_mempool)]));
        self
    }

    /// Queue a `gettxoutproof` RPC call
    pub fn gettxoutproof(mut self, txids: Value, blockhash: Value) -> Self {
        self.calls.push(("gettxoutproof", vec![json!(txids), json!(blockhash)]));
        self
    }

    /// Queue a `gettxoutsetinfo` RPC call
    pub fn gettxoutsetinfo(
        mut self,
        hash_type: Value,
        hash_or_height: Value,
        use_index: Value,
    ) -> Self {
        self.calls.push((
            "gettxoutsetinfo",
            vec![json!(hash_type), json!(hash_or_height), json!(use_index)],
        ));
        self
    }

    /// Queue a `gettxspendingprevout` RPC call
    pub fn gettxspendingprevout(mut self, outputs: Value) -> Self {
        self.calls.push(("gettxspendingprevout", vec![json!(outputs)]));
        self
    }

    /// Queue a `getwalletinfo` RPC call
    pub fn getwalletinfo(mut self) -> Self {
        self.calls.push(("getwalletinfo", Vec::new()));
        self
    }

    /// Queue a `getzmqnotifications` RPC call
    pub fn getzmqnotifications(mut self) -> Self {
        self.calls.push(("getzmqnotifications", Vec::new()));
        self
    }

    /// Queue a `help` RPC call
    pub fn help(mut self, command: Value) -> Self {
        self.calls.push(("help", vec![json!(command)]));
        self
    }

    /// Queue a `importdescriptors` RPC call
    pub fn importdescriptors(mut self, requests: Value) -> Self {
        self.calls.push(("importdescriptors", vec![json!(requests)]));
        self
    }

    /// Queue a `importmempool` RPC call
    pub fn importmempool(mut self, filepath: Value, options: Value) -> Self {
        self.calls.push(("importmempool", vec![json!(filepath), json!(options)]));
        self
    }

    /// Queue a `importprunedfunds` RPC call
    pub fn importprunedfunds(mut self, rawtransaction: Value, txoutproof: Value) -> Self {
        self.calls.push(("importprunedfunds", vec![json!(rawtransaction), json!(txoutproof)]));
        self
    }

    /// Queue a `invalidateblock` RPC call
    pub fn invalidateblock(mut self, blockhash: Value) -> Self {
        self.calls.push(("invalidateblock", vec![json!(blockhash)]));
        self
    }

    /// Queue a `joinpsbts` RPC call
    pub fn joinpsbts(mut self, txs: Value) -> Self {
        self.calls.push(("joinpsbts", vec![json!(txs)]));
        self
    }

    /// Queue a `keypoolrefill` RPC call
    pub fn keypoolrefill(mut self, newsize: Value) -> Self {
        self.calls.push(("keypoolrefill", vec![json!(newsize)]));
        self
    }

    /// Queue a `listaddressgroupings` RPC call
    pub fn listaddressgroupings(mut self) -> Self {
        self.calls.push(("listaddressgroupings", Vec::new()));
        self
    }

    /// Queue a `listbanned` RPC call
    pub fn listbanned(mut self) -> Self {
        self.calls.push(("listbanned", Vec::new()));
        self
    }

    /// Queue a `listdescriptors` RPC call
    pub fn listdescriptors(mut self, private: Value) -> Self {
        self.calls.push(("listdescriptors", vec![json!(private)]));
        self
    }

    /// Queue a `listlabels` RPC call
    pub fn listlabels(mut self, purpose: Value) -> Self {
        self.calls.push(("listlabels", vec![json!(purpose)]));
        self
    }

    /// Queue a `listlockunspent` RPC call
    pub fn listlockunspent(mut self) -> Self {
        self.calls.push(("listlockunspent", Vec::new()));
        self
    }

    /// Queue a `listreceivedbyaddress` RPC call
    pub fn listreceivedbyaddress(
        mut self,
        minconf: Value,
        include_empty: Value,
        include_watchonly: Value,
        address_filter: Value,
        include_immature_coinbase: Value,
    ) -> Self {
        self.calls.push((
            "listreceivedbyaddress",
            vec![
                json!(minconf),
                json!(include_empty),
                json!(include_watchonly),
                json!(address_filter),
                json!(include_immature_coinbase),
            ],
        ));
        self
    }

    /// Queue a `listreceivedbylabel` RPC call
    pub fn listreceivedbylabel(
        mut self,
        minconf: Value,
        include_empty: Value,
        include_watchonly: Value,
        include_immature_coinbase: Value,
    ) -> Self {
        self.calls.push((
            "listreceivedbylabel",
            vec![
                json!(minconf),
                json!(include_empty),
                json!(include_watchonly),
                json!(include_immature_coinbase),
            ],
        ));
        self
    }

    /// Queue a `listsinceblock` RPC call
    pub fn listsinceblock(
        mut self,
        blockhash: Value,
        target_confirmations: Value,
        include_watchonly: Value,
        include_removed: Value,
        include_change: Value,
        label: Value,
    ) -> Self {
        self.calls.push((
            "listsinceblock",
            vec![
                json!(blockhash),
                json!(target_confirmations),
                json!(include_watchonly),
                json!(include_removed),
                json!(include_change),
                json!(label),
            ],
        ));
        self
    }

    /// Queue a `listtransactions` RPC call
    pub fn listtransactions(
        mut self,
        label: Value,
        count: Value,
        skip: Value,
        include_watchonly: Value,
    ) -> Self {
        self.calls.push((
            "listtransactions",
            vec![json!(label), json!(count), json!(skip), json!(include_watchonly)],
        ));
        self
    }

    /// Queue a `listunspent` RPC call
    pub fn listunspent(
        mut self,
        minconf: Value,
        maxconf: Value,
        addresses: Value,
        include_unsafe: Value,
        query_options: Value,
    ) -> Self {
        self.calls.push((
            "listunspent",
            vec![
                json!(minconf),
                json!(maxconf),
                json!(addresses),
                json!(include_unsafe),
                json!(query_options),
            ],
        ));
        self
    }

    /// Queue a `listwalletdir` RPC call
    pub fn listwalletdir(mut self) -> Self {
        self.calls.push(("listwalletdir", Vec::new()));
        self
    }

    /// Queue a `listwallets` RPC call
    pub fn listwallets(mut self) -> Self {
        self.calls.push(("listwallets", Vec::new()));
        self
    }

    /// Queue a `loadtxoutset` RPC call
    pub fn loadtxoutset(mut self, path: Value) -> Self {
        self.calls.push(("loadtxoutset", vec![json!(path)]));
        self
    }

    /// Queue a `loadwallet` RPC call
    pub fn loadwallet(mut self, filename: Value, load_on_startup: Value) -> Self {
        self.calls.push(("loadwallet", vec![json!(filename), json!(load_on_startup)]));
        self
    }

    /// Queue a `lockunspent` RPC call
    pub fn lockunspent(mut self, unlock: Value, transactions: Value, persistent: Value) -> Self {
        self.calls
            .push(("lockunspent", vec![json!(unlock), json!(transactions), json!(persistent)]));
        self
    }

    /// Queue a `logging` RPC call
    pub fn logging(mut self, include: Value, exclude: Value) -> Self {
        self.calls.push(("logging", vec![json!(include), json!(exclude)]));
        self
    }

    /// Queue a `migratewallet` RPC call
    pub fn migratewallet(mut self, wallet_name: Value, passphrase: Value) -> Self {
        self.calls.push(("migratewallet", vec![json!(wallet_name), json!(passphrase)]));
        self
    }

    /// Queue a `mockscheduler` RPC call
    pub fn mockscheduler(mut self, delta_time: Value) -> Self {
        self.calls.push(("mockscheduler", vec![json!(delta_time)]));
        self
    }

    /// Queue a `ping` RPC call
    pub fn ping(mut self) -> Self {
        self.calls.push(("ping", Vec::new()));
        self
    }

    /// Queue a `preciousblock` RPC call
    pub fn preciousblock(mut self, blockhash: Value) -> Self {
        self.calls.push(("preciousblock", vec![json!(blockhash)]));
        self
    }

    /// Queue a `prioritisetransaction` RPC call
    pub fn prioritisetransaction(mut self, txid: Value, dummy: Value, fee_delta: Value) -> Self {
        self.calls
            .push(("prioritisetransaction", vec![json!(txid), json!(dummy), json!(fee_delta)]));
        self
    }

    /// Queue a `pruneblockchain` RPC call
    pub fn pruneblockchain(mut self, height: Value) -> Self {
        self.calls.push(("pruneblockchain", vec![json!(height)]));
        self
    }

    /// Queue a `psbtbumpfee` RPC call
    pub fn psbtbumpfee(mut self, txid: Value, options: Value) -> Self {
        self.calls.push(("psbtbumpfee", vec![json!(txid), json!(options)]));
        self
    }

    /// Queue a `reconsiderblock` RPC call
    pub fn reconsiderblock(mut self, blockhash: Value) -> Self {
        self.calls.push(("reconsiderblock", vec![json!(blockhash)]));
        self
    }

    /// Queue a `removeprunedfunds` RPC call
    pub fn removeprunedfunds(mut self, txid: Value) -> Self {
        self.calls.push(("removeprunedfunds", vec![json!(txid)]));
        self
    }

    /// Queue a `rescanblockchain` RPC call
    pub fn rescanblockchain(mut self, start_height: Value, stop_height: Value) -> Self {
        self.calls.push(("rescanblockchain", vec![json!(start_height), json!(stop_height)]));
        self
    }

    /// Queue a `restorewallet` RPC call
    pub fn restorewallet(
        mut self,
        wallet_name: Value,
        backup_file: Value,
        load_on_startup: Value,
    ) -> Self {
        self.calls.push((
            "restorewallet",
            vec![json!(wallet_name), json!(backup_file), json!(load_on_startup)],
        ));
        self
    }

    /// Queue a `savemempool` RPC call
    pub fn savemempool(mut self) -> Self {
        self.calls.push(("savemempool", Vec::new()));
        self
    }

    /// Queue a `scanblocks` RPC call
    pub fn scanblocks(
        mut self,
        action: Value,
        scanobjects: Value,
        start_height: Value,
        stop_height: Value,
        filtertype: Value,
        options: Value,
    ) -> Self {
        self.calls.push((
            "scanblocks",
            vec![
                json!(action),
                json!(scanobjects),
                json!(start_height),
                json!(stop_height),
                json!(filtertype),
                json!(options),
            ],
        ));
        self
    }

    /// Queue a `scantxoutset` RPC call
    pub fn scantxoutset(mut self, action: Value, scanobjects: Value) -> Self {
        self.calls.push(("scantxoutset", vec![json!(action), json!(scanobjects)]));
        self
    }

    /// Queue a `schema` RPC call
    pub fn schema(mut self) -> Self {
        self.calls.push(("schema", Vec::new()));
        self
    }

    /// Queue a `send` RPC call
    pub fn send(
        mut self,
        outputs: Value,
        conf_target: Value,
        estimate_mode: Value,
        fee_rate: Value,
        options: Value,
        version: Value,
    ) -> Self {
        self.calls.push((
            "send",
            vec![
                json!(outputs),
                json!(conf_target),
                json!(estimate_mode),
                json!(fee_rate),
                json!(options),
                json!(version),
            ],
        ));
        self
    }

    /// Queue a `sendall` RPC call
    pub fn sendall(
        mut self,
        recipients: Value,
        conf_target: Value,
        estimate_mode: Value,
        fee_rate: Value,
        options: Value,
    ) -> Self {
        self.calls.push((
            "sendall",
            vec![
                json!(recipients),
                json!(conf_target),
                json!(estimate_mode),
                json!(fee_rate),
                json!(options),
            ],
        ));
        self
    }

    /// Queue a `sendmany` RPC call
    pub fn sendmany(
        mut self,
        dummy: Value,
        amounts: Value,
        minconf: Value,
        comment: Value,
        subtractfeefrom: Value,
        replaceable: Value,
        conf_target: Value,
        estimate_mode: Value,
        fee_rate: Value,
        verbose: Value,
    ) -> Self {
        self.calls.push((
            "sendmany",
            vec![
                json!(dummy),
                json!(amounts),
                json!(minconf),
                json!(comment),
                json!(subtractfeefrom),
                json!(replaceable),
                json!(conf_target),
                json!(estimate_mode),
                json!(fee_rate),
                json!(verbose),
            ],
        ));
        self
    }

    /// Queue a `sendmsgtopeer` RPC call
    pub fn sendmsgtopeer(mut self, peer_id: Value, msg_type: Value, msg: Value) -> Self {
        self.calls.push(("sendmsgtopeer", vec![json!(peer_id), json!(msg_type), json!(msg)]));
        self
    }

    /// Queue a `sendrawtransaction` RPC call
    pub fn sendrawtransaction(
        mut self,
        hexstring: Value,
        maxfeerate: Value,
        maxburnamount: Value,
    ) -> Self {
        self.calls.push((
            "sendrawtransaction",
            vec![json!(hexstring), json!(maxfeerate), json!(maxburnamount)],
        ));
        self
    }

    /// Queue a `sendtoaddress` RPC call
    pub fn sendtoaddress(
        mut self,
        address: Value,
        amount: Value,
        comment: Value,
        comment_to: Value,
        subtractfeefromamount: Value,
        replaceable: Value,
        conf_target: Value,
        estimate_mode: Value,
        avoid_reuse: Value,
        fee_rate: Value,
        verbose: Value,
    ) -> Self {
        self.calls.push((
            "sendtoaddress",
            vec![
                json!(address),
                json!(amount),
                json!(comment),
                json!(comment_to),
                json!(subtractfeefromamount),
                json!(replaceable),
                json!(conf_target),
                json!(estimate_mode),
                json!(avoid_reuse),
                json!(fee_rate),
                json!(verbose),
            ],
        ));
        self
    }

    /// Queue a `setban` RPC call
    pub fn setban(
        mut self,
        subnet: Value,
        command: Value,
        bantime: Value,
        absolute: Value,
    ) -> Self {
        self.calls
            .push(("setban", vec![json!(subnet), json!(command), json!(bantime), json!(absolute)]));
        self
    }

    /// Queue a `setlabel` RPC call
    pub fn setlabel(mut self, address: Value, label: Value) -> Self {
        self.calls.push(("setlabel", vec![json!(address), json!(label)]));
        self
    }

    /// Queue a `setmocktime` RPC call
    pub fn setmocktime(mut self, timestamp: Value) -> Self {
        self.calls.push(("setmocktime", vec![json!(timestamp)]));
        self
    }

    /// Queue a `setnetworkactive` RPC call
    pub fn setnetworkactive(mut self, state: Value) -> Self {
        self.calls.push(("setnetworkactive", vec![json!(state)]));
        self
    }

    /// Queue a `settxfee` RPC call
    pub fn settxfee(mut self, amount: Value) -> Self {
        self.calls.push(("settxfee", vec![json!(amount)]));
        self
    }

    /// Queue a `setwalletflag` RPC call
    pub fn setwalletflag(mut self, flag: Value, value: Value) -> Self {
        self.calls.push(("setwalletflag", vec![json!(flag), json!(value)]));
        self
    }

    /// Queue a `signmessage` RPC call
    pub fn signmessage(mut self, address: Value, message: Value) -> Self {
        self.calls.push(("signmessage", vec![json!(address), json!(message)]));
        self
    }

    /// Queue a `signmessagewithprivkey` RPC call
    pub fn signmessagewithprivkey(mut self, privkey: Value, message: Value) -> Self {
        self.calls.push(("signmessagewithprivkey", vec![json!(privkey), json!(message)]));
        self
    }

    /// Queue a `signrawtransactionwithkey` RPC call
    pub fn signrawtransactionwithkey(
        mut self,
        hexstring: Value,
        privkeys: Value,
        prevtxs: Value,
        sighashtype: Value,
    ) -> Self {
        self.calls.push((
            "signrawtransactionwithkey",
            vec![json!(hexstring), json!(privkeys), json!(prevtxs), json!(sighashtype)],
        ));
        self
    }

    /// Queue a `signrawtransactionwithwallet` RPC call
    pub fn signrawtransactionwithwallet(
        mut self,
        hexstring: Value,
        prevtxs: Value,
        sighashtype: Value,
    ) -> Self {
        self.calls.push((
            "signrawtransactionwithwallet",
            vec![json!(hexstring), json!(prevtxs), json!(sighashtype)],
        ));
        self
    }

    /// Queue a `simulaterawtransaction` RPC call
    pub fn simulaterawtransaction(mut self, rawtxs: Value, options: Value) -> Self {
        self.calls.push(("simulaterawtransaction", vec![json!(rawtxs), json!(options)]));
        self
    }

    /// Queue a `stop` RPC call
    pub fn stop(mut self, wait: Value) -> Self {
        self.calls.push(("stop", vec![json!(wait)]));
        self
    }

    /// Queue a `submitblock` RPC call
    pub fn submitblock(mut self, hexdata: Value, dummy: Value) -> Self {
        self.calls.push(("submitblock", vec![json!(hexdata), json!(dummy)]));
        self
    }

    /// Queue a `submitheader` RPC call
    pub fn submitheader(mut self, hexdata: Value) -> Self {
        self.calls.push(("submitheader", vec![json!(hexdata)]));
        self
    }

    /// Queue a `submitpackage` RPC call
    pub fn submitpackage(
        mut self,
        package: Value,
        maxfeerate: Value,
        maxburnamount: Value,
    ) -> Self {
        self.calls
            .push(("submitpackage", vec![json!(package), json!(maxfeerate), json!(maxburnamount)]));
        self
    }

    /// Queue a `syncwithvalidationinterfacequeue` RPC call
    pub fn syncwithvalidationinterfacequeue(mut self) -> Self {
        self.calls.push(("syncwithvalidationinterfacequeue", Vec::new()));
        self
    }

    /// Queue a `testmempoolaccept` RPC call
    pub fn testmempoolaccept(mut self, rawtxs: Value, maxfeerate: Value) -> Self {
        self.calls.push(("testmempoolaccept", vec![json!(rawtxs), json!(maxfeerate)]));
        self
    }

    /// Queue a `unloadwallet` RPC call
    pub fn unloadwallet(mut self, wallet_name: Value, load_on_startup: Value) -> Self {
        self.calls.push(("unloadwallet", vec![json!(wallet_name), json!(load_on_startup)]));
        self
    }

    /// Queue a `uptime` RPC call
    pub fn uptime(mut self) -> Self {
        self.calls.push(("uptime", Vec::new()));
        self
    }

    /// Queue a `utxoupdatepsbt` RPC call
    pub fn utxoupdatepsbt(mut self, psbt: Value, descriptors: Value) -> Self {
        self.calls.push(("utxoupdatepsbt", vec![json!(psbt), json!(descriptors)]));
        self
    }

    /// Queue a `validateaddress` RPC call
    pub fn validateaddress(mut self, address: Value) -> Self {
        self.calls.push(("validateaddress", vec![json!(address)]));
        self
    }

    /// Queue a `verifychain` RPC call
    pub fn verifychain(mut self, checklevel: Value, nblocks: Value) -> Self {
        self.calls.push(("verifychain", vec![json!(checklevel), json!(nblocks)]));
        self
    }

    /// Queue a `verifymessage` RPC call
    pub fn verifymessage(mut self, address: Value, signature: Value, message: Value) -> Self {
        self.calls.push(("verifymessage", vec![json!(address), json!(signature), json!(message)]));
        self
    }

    /// Queue a `verifytxoutproof` RPC call
    pub fn verifytxoutproof(mut self, proof: Value) -> Self {
        self.calls.push(("verifytxoutproof", vec![json!(proof)]));
        self
    }

    /// Queue a `waitforblock` RPC call
    pub fn waitforblock(mut self, blockhash: Value, timeout: Value) -> Self {
        self.calls.push(("waitforblock", vec![json!(blockhash), json!(timeout)]));
        self
    }

    /// Queue a `waitforblockheight` RPC call
    pub fn waitforblockheight(mut self, height: Value, timeout: Value) -> Self {
        self.calls.push(("waitforblockheight", vec![json!(height), json!(timeout)]));
        self
    }

    /// Queue a `waitfornewblock` RPC call
    pub fn waitfornewblock(mut self, timeout: Value, current_tip: Value) -> Self {
        self.calls.push(("waitfornewblock", vec![json!(timeout), json!(current_tip)]));
        self
    }

    /// Queue a `walletcreatefundedpsbt` RPC call
    pub fn walletcreatefundedpsbt(
        mut self,
        inputs: Value,
        outputs: Value,
        locktime: Value,
        options: Value,
        bip32derivs: Value,
        version: Value,
    ) -> Self {
        self.calls.push((
            "walletcreatefundedpsbt",
            vec![
                json!(inputs),
                json!(outputs),
                json!(locktime),
                json!(options),
                json!(bip32derivs),
                json!(version),
            ],
        ));
        self
    }

    /// Queue a `walletdisplayaddress` RPC call
    pub fn walletdisplayaddress(mut self, address: Value) -> Self {
        self.calls.push(("walletdisplayaddress", vec![json!(address)]));
        self
    }

    /// Queue a `walletlock` RPC call
    pub fn walletlock(mut self) -> Self {
        self.calls.push(("walletlock", Vec::new()));
        self
    }

    /// Queue a `walletpassphrase` RPC call
    pub fn walletpassphrase(mut self, passphrase: Value, timeout: Value) -> Self {
        self.calls.push(("walletpassphrase", vec![json!(passphrase), json!(timeout)]));
        self
    }

    /// Queue a `walletpassphrasechange` RPC call
    pub fn walletpassphrasechange(mut self, oldpassphrase: Value, newpassphrase: Value) -> Self {
        self.calls
            .push(("walletpassphrasechange", vec![json!(oldpassphrase), json!(newpassphrase)]));
        self
    }

    /// Queue a `walletprocesspsbt` RPC call
    pub fn walletprocesspsbt(
        mut self,
        psbt: Value,
        sign: Value,
        sighashtype: Value,
        bip32derivs: Value,
        finalize: Value,
    ) -> Self {
        self.calls.push((
            "walletprocesspsbt",
            vec![json!(psbt), json!(sign), json!(sighashtype), json!(bip32derivs), json!(finalize)],
        ));
        self
    }

    /// Executes the batch and returns typed results
    pub async fn execute(self) -> Result<BatchResults, TransportError> {
        let BatchBuilder { tx, calls } = self;
        // queue all calls into the transport
        for (method, params) in &calls {
            std::mem::drop(tx.send_request(method, params));
        }
        let raw_results = tx.end_batch().await.map_err(|e| TransportError::Rpc(e.to_string()))?;

        // Parse the raw results into our typed struct
        let mut results = BatchResults {
            abandontransaction: (),
            abortrescan: None,
            addconnection: None,
            addnode: (),
            addpeeraddress: None,
            analyzepsbt: None,
            backupwallet: (),
            bumpfee: None,
            clearbanned: (),
            combinepsbt: None,
            combinerawtransaction: None,
            converttopsbt: None,
            createmultisig: None,
            createpsbt: None,
            createrawtransaction: None,
            createwallet: None,
            createwalletdescriptor: None,
            decodepsbt: None,
            decoderawtransaction: None,
            decodescript: None,
            deriveaddresses: None,
            descriptorprocesspsbt: None,
            disconnectnode: (),
            dumptxoutset: None,
            echo: None,
            echoipc: None,
            echojson: None,
            encryptwallet: None,
            enumeratesigners: None,
            estimaterawfee: None,
            estimatesmartfee: None,
            finalizepsbt: None,
            fundrawtransaction: None,
            generate: (),
            generateblock: None,
            generatetoaddress: None,
            generatetodescriptor: None,
            getaddednodeinfo: None,
            getaddressesbylabel: None,
            getaddressinfo: None,
            getaddrmaninfo: None,
            getbalance: None,
            getbalances: None,
            getbestblockhash: None,
            getblock: None,
            getblockchaininfo: None,
            getblockcount: None,
            getblockfilter: None,
            getblockfrompeer: None,
            getblockhash: None,
            getblockheader: None,
            getblockstats: None,
            getblocktemplate: None,
            getchainstates: None,
            getchaintips: None,
            getchaintxstats: None,
            getconnectioncount: None,
            getdeploymentinfo: None,
            getdescriptoractivity: None,
            getdescriptorinfo: None,
            getdifficulty: None,
            gethdkeys: None,
            getindexinfo: None,
            getmemoryinfo: None,
            getmempoolancestors: None,
            getmempooldescendants: None,
            getmempoolentry: None,
            getmempoolinfo: None,
            getmininginfo: None,
            getnettotals: None,
            getnetworkhashps: None,
            getnetworkinfo: None,
            getnewaddress: None,
            getnodeaddresses: None,
            getorphantxs: None,
            getpeerinfo: None,
            getprioritisedtransactions: None,
            getrawaddrman: None,
            getrawchangeaddress: None,
            getrawmempool: None,
            getrawtransaction: None,
            getreceivedbyaddress: None,
            getreceivedbylabel: None,
            getrpcinfo: None,
            gettransaction: None,
            gettxout: None,
            gettxoutproof: None,
            gettxoutsetinfo: None,
            gettxspendingprevout: None,
            getwalletinfo: None,
            getzmqnotifications: None,
            help: None,
            importdescriptors: None,
            importmempool: None,
            importprunedfunds: (),
            invalidateblock: (),
            joinpsbts: None,
            keypoolrefill: (),
            listaddressgroupings: None,
            listbanned: None,
            listdescriptors: None,
            listlabels: None,
            listlockunspent: None,
            listreceivedbyaddress: None,
            listreceivedbylabel: None,
            listsinceblock: None,
            listtransactions: None,
            listunspent: None,
            listwalletdir: None,
            listwallets: None,
            loadtxoutset: None,
            loadwallet: None,
            lockunspent: None,
            logging: None,
            migratewallet: None,
            mockscheduler: (),
            ping: (),
            preciousblock: (),
            prioritisetransaction: None,
            pruneblockchain: None,
            psbtbumpfee: None,
            reconsiderblock: (),
            removeprunedfunds: (),
            rescanblockchain: None,
            restorewallet: None,
            savemempool: None,
            scanblocks: None,
            scantxoutset: None,
            schema: None,
            send: None,
            sendall: None,
            sendmany: None,
            sendmsgtopeer: None,
            sendrawtransaction: None,
            sendtoaddress: None,
            setban: (),
            setlabel: (),
            setmocktime: (),
            setnetworkactive: None,
            settxfee: None,
            setwalletflag: None,
            signmessage: None,
            signmessagewithprivkey: None,
            signrawtransactionwithkey: None,
            signrawtransactionwithwallet: None,
            simulaterawtransaction: None,
            stop: None,
            submitblock: None,
            submitheader: (),
            submitpackage: None,
            syncwithvalidationinterfacequeue: (),
            testmempoolaccept: None,
            unloadwallet: None,
            uptime: None,
            utxoupdatepsbt: None,
            validateaddress: None,
            verifychain: None,
            verifymessage: None,
            verifytxoutproof: None,
            waitforblock: None,
            waitforblockheight: None,
            waitfornewblock: None,
            walletcreatefundedpsbt: None,
            walletdisplayaddress: None,
            walletlock: (),
            walletpassphrase: (),
            walletpassphrasechange: (),
            walletprocesspsbt: None,
        };

        // Populate the fields based on the actual calls made
        for (i, (method_name, _)) in calls.iter().enumerate() {
            match *method_name {
                "abandontransaction" => results.abandontransaction = (),
                "abortrescan" =>
                    results.abortrescan =
                        Some(serde_json::from_value::<AbortrescanResponse>(raw_results[i].clone())?),
                "addconnection" =>
                    results.addconnection = Some(serde_json::from_value::<AddconnectionResponse>(
                        raw_results[i].clone(),
                    )?),
                "addnode" => results.addnode = (),
                "addpeeraddress" =>
                    results.addpeeraddress = Some(serde_json::from_value::<AddpeeraddressResponse>(
                        raw_results[i].clone(),
                    )?),
                "analyzepsbt" =>
                    results.analyzepsbt =
                        Some(serde_json::from_value::<AnalyzepsbtResponse>(raw_results[i].clone())?),
                "backupwallet" => results.backupwallet = (),
                "bumpfee" =>
                    results.bumpfee =
                        Some(serde_json::from_value::<BumpfeeResponse>(raw_results[i].clone())?),
                "clearbanned" => results.clearbanned = (),
                "combinepsbt" =>
                    results.combinepsbt =
                        Some(serde_json::from_value::<CombinepsbtResponse>(raw_results[i].clone())?),
                "combinerawtransaction" =>
                    results.combinerawtransaction =
                        Some(serde_json::from_value::<CombinerawtransactionResponse>(
                            raw_results[i].clone(),
                        )?),
                "converttopsbt" =>
                    results.converttopsbt = Some(serde_json::from_value::<ConverttopsbtResponse>(
                        raw_results[i].clone(),
                    )?),
                "createmultisig" =>
                    results.createmultisig = Some(serde_json::from_value::<CreatemultisigResponse>(
                        raw_results[i].clone(),
                    )?),
                "createpsbt" =>
                    results.createpsbt =
                        Some(serde_json::from_value::<CreatepsbtResponse>(raw_results[i].clone())?),
                "createrawtransaction" =>
                    results.createrawtransaction =
                        Some(serde_json::from_value::<CreaterawtransactionResponse>(
                            raw_results[i].clone(),
                        )?),
                "createwallet" =>
                    results.createwallet = Some(serde_json::from_value::<CreatewalletResponse>(
                        raw_results[i].clone(),
                    )?),
                "createwalletdescriptor" =>
                    results.createwalletdescriptor =
                        Some(serde_json::from_value::<CreatewalletdescriptorResponse>(
                            raw_results[i].clone(),
                        )?),
                "decodepsbt" =>
                    results.decodepsbt =
                        Some(serde_json::from_value::<DecodepsbtResponse>(raw_results[i].clone())?),
                "decoderawtransaction" =>
                    results.decoderawtransaction =
                        Some(serde_json::from_value::<DecoderawtransactionResponse>(
                            raw_results[i].clone(),
                        )?),
                "decodescript" =>
                    results.decodescript = Some(serde_json::from_value::<DecodescriptResponse>(
                        raw_results[i].clone(),
                    )?),
                "deriveaddresses" =>
                    results.deriveaddresses = Some(
                        serde_json::from_value::<DeriveaddressesResponse>(raw_results[i].clone())?,
                    ),
                "descriptorprocesspsbt" =>
                    results.descriptorprocesspsbt =
                        Some(serde_json::from_value::<DescriptorprocesspsbtResponse>(
                            raw_results[i].clone(),
                        )?),
                "disconnectnode" => results.disconnectnode = (),
                "dumptxoutset" =>
                    results.dumptxoutset = Some(serde_json::from_value::<DumptxoutsetResponse>(
                        raw_results[i].clone(),
                    )?),
                "echo" =>
                    results.echo =
                        Some(serde_json::from_value::<EchoResponse>(raw_results[i].clone())?),
                "echoipc" =>
                    results.echoipc =
                        Some(serde_json::from_value::<EchoipcResponse>(raw_results[i].clone())?),
                "echojson" =>
                    results.echojson =
                        Some(serde_json::from_value::<EchojsonResponse>(raw_results[i].clone())?),
                "encryptwallet" =>
                    results.encryptwallet = Some(serde_json::from_value::<EncryptwalletResponse>(
                        raw_results[i].clone(),
                    )?),
                "enumeratesigners" =>
                    results.enumeratesigners = Some(serde_json::from_value::<
                        EnumeratesignersResponse,
                    >(raw_results[i].clone())?),
                "estimaterawfee" =>
                    results.estimaterawfee = Some(serde_json::from_value::<EstimaterawfeeResponse>(
                        raw_results[i].clone(),
                    )?),
                "estimatesmartfee" =>
                    results.estimatesmartfee = Some(serde_json::from_value::<
                        EstimatesmartfeeResponse,
                    >(raw_results[i].clone())?),
                "finalizepsbt" =>
                    results.finalizepsbt = Some(serde_json::from_value::<FinalizepsbtResponse>(
                        raw_results[i].clone(),
                    )?),
                "fundrawtransaction" =>
                    results.fundrawtransaction = Some(serde_json::from_value::<
                        FundrawtransactionResponse,
                    >(raw_results[i].clone())?),
                "generate" => results.generate = (),
                "generateblock" =>
                    results.generateblock = Some(serde_json::from_value::<GenerateblockResponse>(
                        raw_results[i].clone(),
                    )?),
                "generatetoaddress" =>
                    results.generatetoaddress = Some(serde_json::from_value::<
                        GeneratetoaddressResponse,
                    >(raw_results[i].clone())?),
                "generatetodescriptor" =>
                    results.generatetodescriptor =
                        Some(serde_json::from_value::<GeneratetodescriptorResponse>(
                            raw_results[i].clone(),
                        )?),
                "getaddednodeinfo" =>
                    results.getaddednodeinfo = Some(serde_json::from_value::<
                        GetaddednodeinfoResponse,
                    >(raw_results[i].clone())?),
                "getaddressesbylabel" =>
                    results.getaddressesbylabel =
                        Some(serde_json::from_value::<GetaddressesbylabelResponse>(
                            raw_results[i].clone(),
                        )?),
                "getaddressinfo" =>
                    results.getaddressinfo = Some(serde_json::from_value::<GetaddressinfoResponse>(
                        raw_results[i].clone(),
                    )?),
                "getaddrmaninfo" =>
                    results.getaddrmaninfo = Some(serde_json::from_value::<GetaddrmaninfoResponse>(
                        raw_results[i].clone(),
                    )?),
                "getbalance" =>
                    results.getbalance =
                        Some(serde_json::from_value::<GetbalanceResponse>(raw_results[i].clone())?),
                "getbalances" =>
                    results.getbalances =
                        Some(serde_json::from_value::<GetbalancesResponse>(raw_results[i].clone())?),
                "getbestblockhash" =>
                    results.getbestblockhash = Some(serde_json::from_value::<
                        GetbestblockhashResponse,
                    >(raw_results[i].clone())?),
                "getblock" =>
                    results.getblock =
                        Some(serde_json::from_value::<GetblockResponse>(raw_results[i].clone())?),
                "getblockchaininfo" =>
                    results.getblockchaininfo = Some(serde_json::from_value::<
                        GetblockchaininfoResponse,
                    >(raw_results[i].clone())?),
                "getblockcount" =>
                    results.getblockcount = Some(serde_json::from_value::<GetblockcountResponse>(
                        raw_results[i].clone(),
                    )?),
                "getblockfilter" =>
                    results.getblockfilter = Some(serde_json::from_value::<GetblockfilterResponse>(
                        raw_results[i].clone(),
                    )?),
                "getblockfrompeer" =>
                    results.getblockfrompeer = Some(serde_json::from_value::<
                        GetblockfrompeerResponse,
                    >(raw_results[i].clone())?),
                "getblockhash" =>
                    results.getblockhash = Some(serde_json::from_value::<GetblockhashResponse>(
                        raw_results[i].clone(),
                    )?),
                "getblockheader" =>
                    results.getblockheader = Some(serde_json::from_value::<GetblockheaderResponse>(
                        raw_results[i].clone(),
                    )?),
                "getblockstats" =>
                    results.getblockstats = Some(serde_json::from_value::<GetblockstatsResponse>(
                        raw_results[i].clone(),
                    )?),
                "getblocktemplate" =>
                    results.getblocktemplate = Some(serde_json::from_value::<
                        GetblocktemplateResponse,
                    >(raw_results[i].clone())?),
                "getchainstates" =>
                    results.getchainstates = Some(serde_json::from_value::<GetchainstatesResponse>(
                        raw_results[i].clone(),
                    )?),
                "getchaintips" =>
                    results.getchaintips = Some(serde_json::from_value::<GetchaintipsResponse>(
                        raw_results[i].clone(),
                    )?),
                "getchaintxstats" =>
                    results.getchaintxstats = Some(
                        serde_json::from_value::<GetchaintxstatsResponse>(raw_results[i].clone())?,
                    ),
                "getconnectioncount" =>
                    results.getconnectioncount = Some(serde_json::from_value::<
                        GetconnectioncountResponse,
                    >(raw_results[i].clone())?),
                "getdeploymentinfo" =>
                    results.getdeploymentinfo = Some(serde_json::from_value::<
                        GetdeploymentinfoResponse,
                    >(raw_results[i].clone())?),
                "getdescriptoractivity" =>
                    results.getdescriptoractivity =
                        Some(serde_json::from_value::<GetdescriptoractivityResponse>(
                            raw_results[i].clone(),
                        )?),
                "getdescriptorinfo" =>
                    results.getdescriptorinfo = Some(serde_json::from_value::<
                        GetdescriptorinfoResponse,
                    >(raw_results[i].clone())?),
                "getdifficulty" =>
                    results.getdifficulty = Some(serde_json::from_value::<GetdifficultyResponse>(
                        raw_results[i].clone(),
                    )?),
                "gethdkeys" =>
                    results.gethdkeys =
                        Some(serde_json::from_value::<GethdkeysResponse>(raw_results[i].clone())?),
                "getindexinfo" =>
                    results.getindexinfo = Some(serde_json::from_value::<GetindexinfoResponse>(
                        raw_results[i].clone(),
                    )?),
                "getmemoryinfo" =>
                    results.getmemoryinfo = Some(serde_json::from_value::<GetmemoryinfoResponse>(
                        raw_results[i].clone(),
                    )?),
                "getmempoolancestors" =>
                    results.getmempoolancestors =
                        Some(serde_json::from_value::<GetmempoolancestorsResponse>(
                            raw_results[i].clone(),
                        )?),
                "getmempooldescendants" =>
                    results.getmempooldescendants =
                        Some(serde_json::from_value::<GetmempooldescendantsResponse>(
                            raw_results[i].clone(),
                        )?),
                "getmempoolentry" =>
                    results.getmempoolentry = Some(
                        serde_json::from_value::<GetmempoolentryResponse>(raw_results[i].clone())?,
                    ),
                "getmempoolinfo" =>
                    results.getmempoolinfo = Some(serde_json::from_value::<GetmempoolinfoResponse>(
                        raw_results[i].clone(),
                    )?),
                "getmininginfo" =>
                    results.getmininginfo = Some(serde_json::from_value::<GetmininginfoResponse>(
                        raw_results[i].clone(),
                    )?),
                "getnettotals" =>
                    results.getnettotals = Some(serde_json::from_value::<GetnettotalsResponse>(
                        raw_results[i].clone(),
                    )?),
                "getnetworkhashps" =>
                    results.getnetworkhashps = Some(serde_json::from_value::<
                        GetnetworkhashpsResponse,
                    >(raw_results[i].clone())?),
                "getnetworkinfo" =>
                    results.getnetworkinfo = Some(serde_json::from_value::<GetnetworkinfoResponse>(
                        raw_results[i].clone(),
                    )?),
                "getnewaddress" =>
                    results.getnewaddress = Some(serde_json::from_value::<GetnewaddressResponse>(
                        raw_results[i].clone(),
                    )?),
                "getnodeaddresses" =>
                    results.getnodeaddresses = Some(serde_json::from_value::<
                        GetnodeaddressesResponse,
                    >(raw_results[i].clone())?),
                "getorphantxs" =>
                    results.getorphantxs = Some(serde_json::from_value::<GetorphantxsResponse>(
                        raw_results[i].clone(),
                    )?),
                "getpeerinfo" =>
                    results.getpeerinfo =
                        Some(serde_json::from_value::<GetpeerinfoResponse>(raw_results[i].clone())?),
                "getprioritisedtransactions" =>
                    results.getprioritisedtransactions =
                        Some(serde_json::from_value::<GetprioritisedtransactionsResponse>(
                            raw_results[i].clone(),
                        )?),
                "getrawaddrman" =>
                    results.getrawaddrman = Some(serde_json::from_value::<GetrawaddrmanResponse>(
                        raw_results[i].clone(),
                    )?),
                "getrawchangeaddress" =>
                    results.getrawchangeaddress =
                        Some(serde_json::from_value::<GetrawchangeaddressResponse>(
                            raw_results[i].clone(),
                        )?),
                "getrawmempool" =>
                    results.getrawmempool = Some(serde_json::from_value::<GetrawmempoolResponse>(
                        raw_results[i].clone(),
                    )?),
                "getrawtransaction" =>
                    results.getrawtransaction = Some(serde_json::from_value::<
                        GetrawtransactionResponse,
                    >(raw_results[i].clone())?),
                "getreceivedbyaddress" =>
                    results.getreceivedbyaddress =
                        Some(serde_json::from_value::<GetreceivedbyaddressResponse>(
                            raw_results[i].clone(),
                        )?),
                "getreceivedbylabel" =>
                    results.getreceivedbylabel = Some(serde_json::from_value::<
                        GetreceivedbylabelResponse,
                    >(raw_results[i].clone())?),
                "getrpcinfo" =>
                    results.getrpcinfo =
                        Some(serde_json::from_value::<GetrpcinfoResponse>(raw_results[i].clone())?),
                "gettransaction" =>
                    results.gettransaction = Some(serde_json::from_value::<GettransactionResponse>(
                        raw_results[i].clone(),
                    )?),
                "gettxout" =>
                    results.gettxout =
                        Some(serde_json::from_value::<GettxoutResponse>(raw_results[i].clone())?),
                "gettxoutproof" =>
                    results.gettxoutproof = Some(serde_json::from_value::<GettxoutproofResponse>(
                        raw_results[i].clone(),
                    )?),
                "gettxoutsetinfo" =>
                    results.gettxoutsetinfo = Some(
                        serde_json::from_value::<GettxoutsetinfoResponse>(raw_results[i].clone())?,
                    ),
                "gettxspendingprevout" =>
                    results.gettxspendingprevout =
                        Some(serde_json::from_value::<GettxspendingprevoutResponse>(
                            raw_results[i].clone(),
                        )?),
                "getwalletinfo" =>
                    results.getwalletinfo = Some(serde_json::from_value::<GetwalletinfoResponse>(
                        raw_results[i].clone(),
                    )?),
                "getzmqnotifications" =>
                    results.getzmqnotifications =
                        Some(serde_json::from_value::<GetzmqnotificationsResponse>(
                            raw_results[i].clone(),
                        )?),
                "help" =>
                    results.help =
                        Some(serde_json::from_value::<HelpResponse>(raw_results[i].clone())?),
                "importdescriptors" =>
                    results.importdescriptors = Some(serde_json::from_value::<
                        ImportdescriptorsResponse,
                    >(raw_results[i].clone())?),
                "importmempool" =>
                    results.importmempool = Some(serde_json::from_value::<ImportmempoolResponse>(
                        raw_results[i].clone(),
                    )?),
                "importprunedfunds" => results.importprunedfunds = (),
                "invalidateblock" => results.invalidateblock = (),
                "joinpsbts" =>
                    results.joinpsbts =
                        Some(serde_json::from_value::<JoinpsbtsResponse>(raw_results[i].clone())?),
                "keypoolrefill" => results.keypoolrefill = (),
                "listaddressgroupings" =>
                    results.listaddressgroupings =
                        Some(serde_json::from_value::<ListaddressgroupingsResponse>(
                            raw_results[i].clone(),
                        )?),
                "listbanned" =>
                    results.listbanned =
                        Some(serde_json::from_value::<ListbannedResponse>(raw_results[i].clone())?),
                "listdescriptors" =>
                    results.listdescriptors = Some(
                        serde_json::from_value::<ListdescriptorsResponse>(raw_results[i].clone())?,
                    ),
                "listlabels" =>
                    results.listlabels =
                        Some(serde_json::from_value::<ListlabelsResponse>(raw_results[i].clone())?),
                "listlockunspent" =>
                    results.listlockunspent = Some(
                        serde_json::from_value::<ListlockunspentResponse>(raw_results[i].clone())?,
                    ),
                "listreceivedbyaddress" =>
                    results.listreceivedbyaddress =
                        Some(serde_json::from_value::<ListreceivedbyaddressResponse>(
                            raw_results[i].clone(),
                        )?),
                "listreceivedbylabel" =>
                    results.listreceivedbylabel =
                        Some(serde_json::from_value::<ListreceivedbylabelResponse>(
                            raw_results[i].clone(),
                        )?),
                "listsinceblock" =>
                    results.listsinceblock = Some(serde_json::from_value::<ListsinceblockResponse>(
                        raw_results[i].clone(),
                    )?),
                "listtransactions" =>
                    results.listtransactions = Some(serde_json::from_value::<
                        ListtransactionsResponse,
                    >(raw_results[i].clone())?),
                "listunspent" =>
                    results.listunspent =
                        Some(serde_json::from_value::<ListunspentResponse>(raw_results[i].clone())?),
                "listwalletdir" =>
                    results.listwalletdir = Some(serde_json::from_value::<ListwalletdirResponse>(
                        raw_results[i].clone(),
                    )?),
                "listwallets" =>
                    results.listwallets =
                        Some(serde_json::from_value::<ListwalletsResponse>(raw_results[i].clone())?),
                "loadtxoutset" =>
                    results.loadtxoutset = Some(serde_json::from_value::<LoadtxoutsetResponse>(
                        raw_results[i].clone(),
                    )?),
                "loadwallet" =>
                    results.loadwallet =
                        Some(serde_json::from_value::<LoadwalletResponse>(raw_results[i].clone())?),
                "lockunspent" =>
                    results.lockunspent =
                        Some(serde_json::from_value::<LockunspentResponse>(raw_results[i].clone())?),
                "logging" =>
                    results.logging =
                        Some(serde_json::from_value::<LoggingResponse>(raw_results[i].clone())?),
                "migratewallet" =>
                    results.migratewallet = Some(serde_json::from_value::<MigratewalletResponse>(
                        raw_results[i].clone(),
                    )?),
                "mockscheduler" => results.mockscheduler = (),
                "ping" => results.ping = (),
                "preciousblock" => results.preciousblock = (),
                "prioritisetransaction" =>
                    results.prioritisetransaction =
                        Some(serde_json::from_value::<PrioritisetransactionResponse>(
                            raw_results[i].clone(),
                        )?),
                "pruneblockchain" =>
                    results.pruneblockchain = Some(
                        serde_json::from_value::<PruneblockchainResponse>(raw_results[i].clone())?,
                    ),
                "psbtbumpfee" =>
                    results.psbtbumpfee =
                        Some(serde_json::from_value::<PsbtbumpfeeResponse>(raw_results[i].clone())?),
                "reconsiderblock" => results.reconsiderblock = (),
                "removeprunedfunds" => results.removeprunedfunds = (),
                "rescanblockchain" =>
                    results.rescanblockchain = Some(serde_json::from_value::<
                        RescanblockchainResponse,
                    >(raw_results[i].clone())?),
                "restorewallet" =>
                    results.restorewallet = Some(serde_json::from_value::<RestorewalletResponse>(
                        raw_results[i].clone(),
                    )?),
                "savemempool" =>
                    results.savemempool =
                        Some(serde_json::from_value::<SavemempoolResponse>(raw_results[i].clone())?),
                "scanblocks" =>
                    results.scanblocks =
                        Some(serde_json::from_value::<ScanblocksResponse>(raw_results[i].clone())?),
                "scantxoutset" =>
                    results.scantxoutset = Some(serde_json::from_value::<ScantxoutsetResponse>(
                        raw_results[i].clone(),
                    )?),
                "schema" =>
                    results.schema =
                        Some(serde_json::from_value::<SchemaResponse>(raw_results[i].clone())?),
                "send" =>
                    results.send =
                        Some(serde_json::from_value::<SendResponse>(raw_results[i].clone())?),
                "sendall" =>
                    results.sendall =
                        Some(serde_json::from_value::<SendallResponse>(raw_results[i].clone())?),
                "sendmany" =>
                    results.sendmany =
                        Some(serde_json::from_value::<SendmanyResponse>(raw_results[i].clone())?),
                "sendmsgtopeer" =>
                    results.sendmsgtopeer = Some(serde_json::from_value::<SendmsgtopeerResponse>(
                        raw_results[i].clone(),
                    )?),
                "sendrawtransaction" =>
                    results.sendrawtransaction = Some(serde_json::from_value::<
                        SendrawtransactionResponse,
                    >(raw_results[i].clone())?),
                "sendtoaddress" =>
                    results.sendtoaddress = Some(serde_json::from_value::<SendtoaddressResponse>(
                        raw_results[i].clone(),
                    )?),
                "setban" => results.setban = (),
                "setlabel" => results.setlabel = (),
                "setmocktime" => results.setmocktime = (),
                "setnetworkactive" =>
                    results.setnetworkactive = Some(serde_json::from_value::<
                        SetnetworkactiveResponse,
                    >(raw_results[i].clone())?),
                "settxfee" =>
                    results.settxfee =
                        Some(serde_json::from_value::<SettxfeeResponse>(raw_results[i].clone())?),
                "setwalletflag" =>
                    results.setwalletflag = Some(serde_json::from_value::<SetwalletflagResponse>(
                        raw_results[i].clone(),
                    )?),
                "signmessage" =>
                    results.signmessage =
                        Some(serde_json::from_value::<SignmessageResponse>(raw_results[i].clone())?),
                "signmessagewithprivkey" =>
                    results.signmessagewithprivkey =
                        Some(serde_json::from_value::<SignmessagewithprivkeyResponse>(
                            raw_results[i].clone(),
                        )?),
                "signrawtransactionwithkey" =>
                    results.signrawtransactionwithkey =
                        Some(serde_json::from_value::<SignrawtransactionwithkeyResponse>(
                            raw_results[i].clone(),
                        )?),
                "signrawtransactionwithwallet" =>
                    results.signrawtransactionwithwallet =
                        Some(serde_json::from_value::<SignrawtransactionwithwalletResponse>(
                            raw_results[i].clone(),
                        )?),
                "simulaterawtransaction" =>
                    results.simulaterawtransaction =
                        Some(serde_json::from_value::<SimulaterawtransactionResponse>(
                            raw_results[i].clone(),
                        )?),
                "stop" =>
                    results.stop =
                        Some(serde_json::from_value::<StopResponse>(raw_results[i].clone())?),
                "submitblock" =>
                    results.submitblock =
                        Some(serde_json::from_value::<SubmitblockResponse>(raw_results[i].clone())?),
                "submitheader" => results.submitheader = (),
                "submitpackage" =>
                    results.submitpackage = Some(serde_json::from_value::<SubmitpackageResponse>(
                        raw_results[i].clone(),
                    )?),
                "syncwithvalidationinterfacequeue" => results.syncwithvalidationinterfacequeue = (),
                "testmempoolaccept" =>
                    results.testmempoolaccept = Some(serde_json::from_value::<
                        TestmempoolacceptResponse,
                    >(raw_results[i].clone())?),
                "unloadwallet" =>
                    results.unloadwallet = Some(serde_json::from_value::<UnloadwalletResponse>(
                        raw_results[i].clone(),
                    )?),
                "uptime" =>
                    results.uptime =
                        Some(serde_json::from_value::<UptimeResponse>(raw_results[i].clone())?),
                "utxoupdatepsbt" =>
                    results.utxoupdatepsbt = Some(serde_json::from_value::<UtxoupdatepsbtResponse>(
                        raw_results[i].clone(),
                    )?),
                "validateaddress" =>
                    results.validateaddress = Some(
                        serde_json::from_value::<ValidateaddressResponse>(raw_results[i].clone())?,
                    ),
                "verifychain" =>
                    results.verifychain =
                        Some(serde_json::from_value::<VerifychainResponse>(raw_results[i].clone())?),
                "verifymessage" =>
                    results.verifymessage = Some(serde_json::from_value::<VerifymessageResponse>(
                        raw_results[i].clone(),
                    )?),
                "verifytxoutproof" =>
                    results.verifytxoutproof = Some(serde_json::from_value::<
                        VerifytxoutproofResponse,
                    >(raw_results[i].clone())?),
                "waitforblock" =>
                    results.waitforblock = Some(serde_json::from_value::<WaitforblockResponse>(
                        raw_results[i].clone(),
                    )?),
                "waitforblockheight" =>
                    results.waitforblockheight = Some(serde_json::from_value::<
                        WaitforblockheightResponse,
                    >(raw_results[i].clone())?),
                "waitfornewblock" =>
                    results.waitfornewblock = Some(
                        serde_json::from_value::<WaitfornewblockResponse>(raw_results[i].clone())?,
                    ),
                "walletcreatefundedpsbt" =>
                    results.walletcreatefundedpsbt =
                        Some(serde_json::from_value::<WalletcreatefundedpsbtResponse>(
                            raw_results[i].clone(),
                        )?),
                "walletdisplayaddress" =>
                    results.walletdisplayaddress =
                        Some(serde_json::from_value::<WalletdisplayaddressResponse>(
                            raw_results[i].clone(),
                        )?),
                "walletlock" => results.walletlock = (),
                "walletpassphrase" => results.walletpassphrase = (),
                "walletpassphrasechange" => results.walletpassphrasechange = (),
                "walletprocesspsbt" =>
                    results.walletprocesspsbt = Some(serde_json::from_value::<
                        WalletprocesspsbtResponse,
                    >(raw_results[i].clone())?),
                _ => return Err(TransportError::Rpc(format!("Unknown method: {}", method_name))),
            }
        }

        Ok(results)
    }
}
