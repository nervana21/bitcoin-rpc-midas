#[cfg(test)]
use anyhow::Result;
use std::sync::Arc;
use crate::transport::core::{TransportExt, TransportError};
use crate::transport::{DefaultTransport};
use crate::types::v28_types::*;
#[cfg(test)]
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct BitcoinWalletClient {
    client: Arc<DefaultTransport>,
}

impl BitcoinWalletClient {
    pub fn new(client: Arc<DefaultTransport>) -> Self {
        Self { client }
    }

    pub fn with_transport(&mut self, client: Arc<DefaultTransport>) {
        self.client = client;
    }

/// Mark in-wallet transaction <txid> as abandoned
/// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
/// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
/// It only works on transactions which are not included in a block and are not currently in the mempool.
/// It has no effect on transactions which are already abandoned.
    pub async fn abandontransaction(&self, txid: bitcoin::Txid) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("abandontransaction", &params).await
    }

/// Stops current wallet rescan triggered by an RPC call, e.g. by an importprivkey call.
/// Note: Use "getwalletinfo" to query the scanning progress.
    pub async fn abortrescan(&self) -> Result<AbortrescanResponse, TransportError> {
        // dispatch and deserialize to `AbortrescanResponse`
        self.client.call::<AbortrescanResponse>("abortrescan", &[]).await
    }

/// Add an nrequired-to-sign multisignature address to the wallet. Requires a new wallet backup.
/// Each key is a Bitcoin address or hex-encoded public key.
/// This functionality is only intended for use with non-watchonly addresses.
/// See ``importaddress`` for watchonly p2sh address support.
/// If "label" is specified, assign address to that label.
/// Note: This command is only compatible with legacy wallets.
    pub async fn addmultisigaddress(&self, nrequired: u32, keys: Vec<String>, label: String, address_type: String) -> Result<AddmultisigaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(nrequired)?);
        params.push(serde_json::to_value(keys)?);
        params.push(serde_json::to_value(label)?);
        params.push(serde_json::to_value(address_type)?);
        // dispatch and deserialize to `AddmultisigaddressResponse`
        self.client.call::<AddmultisigaddressResponse>("addmultisigaddress", &params).await
    }

/// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
    pub async fn backupwallet(&self, destination: String) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(destination)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("backupwallet", &params).await
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
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(options)?);
        // dispatch and deserialize to `BumpfeeResponse`
        self.client.call::<BumpfeeResponse>("bumpfee", &params).await
    }

/// Creates and loads a new wallet.
    pub async fn createwallet(&self, wallet_name: String, disable_private_keys: bool, blank: bool, passphrase: String, avoid_reuse: bool, descriptors: bool, load_on_startup: bool, external_signer: bool) -> Result<CreatewalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(wallet_name)?);
        params.push(serde_json::to_value(disable_private_keys)?);
        params.push(serde_json::to_value(blank)?);
        params.push(serde_json::to_value(passphrase)?);
        params.push(serde_json::to_value(avoid_reuse)?);
        params.push(serde_json::to_value(descriptors)?);
        params.push(serde_json::to_value(load_on_startup)?);
        params.push(serde_json::to_value(external_signer)?);
        // dispatch and deserialize to `CreatewalletResponse`
        self.client.call::<CreatewalletResponse>("createwallet", &params).await
    }

/// Reveals the private key corresponding to "address".
/// Then the importprivkey can be used with this output
/// Note: This command is only compatible with legacy wallets.
    pub async fn dumpprivkey(&self, address: String) -> Result<DumpprivkeyResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        // dispatch and deserialize to `DumpprivkeyResponse`
        self.client.call::<DumpprivkeyResponse>("dumpprivkey", &params).await
    }

/// Dumps all wallet keys in a human-readable format to a server-side file. This does not allow overwriting existing files.
/// Imported scripts are included in the dumpfile, but corresponding BIP173 addresses, etc. may not be added automatically by importwallet.
/// Note that if your wallet contains keys which are not derived from your HD seed (e.g. imported keys), these are not covered by
/// only backing up the seed itself, and must be backed up too (e.g. ensure you back up the whole dumpfile).
/// Note: This command is only compatible with legacy wallets.
    pub async fn dumpwallet(&self, filename: String) -> Result<DumpwalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(filename)?);
        // dispatch and deserialize to `DumpwalletResponse`
        self.client.call::<DumpwalletResponse>("dumpwallet", &params).await
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
        let mut params = Vec::new();
        params.push(serde_json::to_value(passphrase)?);
        // dispatch and deserialize to `EncryptwalletResponse`
        self.client.call::<EncryptwalletResponse>("encryptwallet", &params).await
    }

/// Returns the list of addresses assigned the specified label.
    pub async fn getaddressesbylabel(&self, label: String) -> Result<GetaddressesbylabelResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(label)?);
        // dispatch and deserialize to `GetaddressesbylabelResponse`
        self.client.call::<GetaddressesbylabelResponse>("getaddressesbylabel", &params).await
    }

/// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
    pub async fn getaddressinfo(&self, address: String) -> Result<GetaddressinfoResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        // dispatch and deserialize to `GetaddressinfoResponse`
        self.client.call::<GetaddressinfoResponse>("getaddressinfo", &params).await
    }

/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
    pub async fn getbalance(&self, dummy: Option<String>, minconf: u32, include_watchonly: bool, avoid_reuse: bool) -> Result<GetbalanceResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(dummy)?);
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(include_watchonly)?);
        params.push(serde_json::to_value(avoid_reuse)?);
        // dispatch and deserialize to `GetbalanceResponse`
        self.client.call::<GetbalanceResponse>("getbalance", &params).await
    }

/// Returns an object with all balances in BTC.
    pub async fn getbalances(&self) -> Result<GetbalancesResponse, TransportError> {
        // dispatch and deserialize to `GetbalancesResponse`
        self.client.call::<GetbalancesResponse>("getbalances", &[]).await
    }

/// Returns a new Bitcoin address for receiving payments.
/// If "label" is specified, it is added to the address book
/// so payments received with the address will be associated with "label".
    pub async fn getnewaddress(&self, label: String, address_type: String) -> Result<GetnewaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(label)?);
        params.push(serde_json::to_value(address_type)?);
        // dispatch and deserialize to `GetnewaddressResponse`
        self.client.call::<GetnewaddressResponse>("getnewaddress", &params).await
    }

/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
    pub async fn getrawchangeaddress(&self, address_type: String) -> Result<GetrawchangeaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address_type)?);
        // dispatch and deserialize to `GetrawchangeaddressResponse`
        self.client.call::<GetrawchangeaddressResponse>("getrawchangeaddress", &params).await
    }

/// Returns the total amount received by the given address in transactions with at least minconf confirmations.
    pub async fn getreceivedbyaddress(&self, address: String, minconf: u32, include_immature_coinbase: bool) -> Result<GetreceivedbyaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(include_immature_coinbase)?);
        // dispatch and deserialize to `GetreceivedbyaddressResponse`
        self.client.call::<GetreceivedbyaddressResponse>("getreceivedbyaddress", &params).await
    }

/// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
    pub async fn getreceivedbylabel(&self, label: String, minconf: u32, include_immature_coinbase: bool) -> Result<GetreceivedbylabelResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(label)?);
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(include_immature_coinbase)?);
        // dispatch and deserialize to `GetreceivedbylabelResponse`
        self.client.call::<GetreceivedbylabelResponse>("getreceivedbylabel", &params).await
    }

/// Get detailed information about in-wallet transaction <txid>
    pub async fn gettransaction(&self, txid: bitcoin::Txid, include_watchonly: bool, verbose: bool) -> Result<GettransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(include_watchonly)?);
        params.push(serde_json::to_value(verbose)?);
        // dispatch and deserialize to `GettransactionResponse`
        self.client.call::<GettransactionResponse>("gettransaction", &params).await
    }

/// DEPRECATED
/// Identical to getbalances().mine.untrusted_pending
    pub async fn getunconfirmedbalance(&self) -> Result<GetunconfirmedbalanceResponse, TransportError> {
        // dispatch and deserialize to `GetunconfirmedbalanceResponse`
        self.client.call::<GetunconfirmedbalanceResponse>("getunconfirmedbalance", &[]).await
    }

/// Returns an object containing various wallet state info.
    pub async fn getwalletinfo(&self) -> Result<GetwalletinfoResponse, TransportError> {
        // dispatch and deserialize to `GetwalletinfoResponse`
        self.client.call::<GetwalletinfoResponse>("getwalletinfo", &[]).await
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
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(label)?);
        params.push(serde_json::to_value(rescan)?);
        params.push(serde_json::to_value(p2sh)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("importaddress", &params).await
    }

/// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
/// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second elements will be imported as an internal descriptor.
///
/// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
    pub async fn importdescriptors(&self, requests: Vec<serde_json::Value>) -> Result<ImportdescriptorsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(requests)?);
        // dispatch and deserialize to `ImportdescriptorsResponse`
        self.client.call::<ImportdescriptorsResponse>("importdescriptors", &params).await
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
        let mut params = Vec::new();
        params.push(serde_json::to_value(requests)?);
        params.push(serde_json::to_value(options)?);
        // dispatch and deserialize to `ImportmultiResponse`
        self.client.call::<ImportmultiResponse>("importmulti", &params).await
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
        let mut params = Vec::new();
        params.push(serde_json::to_value(privkey)?);
        params.push(serde_json::to_value(label)?);
        params.push(serde_json::to_value(rescan)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("importprivkey", &params).await
    }

/// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
    pub async fn importprunedfunds(&self, rawtransaction: String, txoutproof: String) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(rawtransaction)?);
        params.push(serde_json::to_value(txoutproof)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("importprunedfunds", &params).await
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
        let mut params = Vec::new();
        params.push(serde_json::to_value(pubkey)?);
        params.push(serde_json::to_value(label)?);
        params.push(serde_json::to_value(rescan)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("importpubkey", &params).await
    }

/// Imports keys from a wallet dump file (see dumpwallet). Requires a new wallet backup to include imported keys.
/// Note: Blockchain and Mempool will be rescanned after a successful import. Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets.
    pub async fn importwallet(&self, filename: String) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(filename)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("importwallet", &params).await
    }

/// Fills the keypool.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn keypoolrefill(&self, newsize: u64) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(newsize)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("keypoolrefill", &params).await
    }

/// Lists groups of addresses which have had their common ownership
/// made public by common use as inputs or as the resulting change
/// in past transactions
    pub async fn listaddressgroupings(&self) -> Result<ListaddressgroupingsResponse, TransportError> {
        // dispatch and deserialize to `ListaddressgroupingsResponse`
        self.client.call::<ListaddressgroupingsResponse>("listaddressgroupings", &[]).await
    }

/// List descriptors imported into a descriptor-enabled wallet.
    pub async fn listdescriptors(&self, private: bool) -> Result<ListdescriptorsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(private)?);
        // dispatch and deserialize to `ListdescriptorsResponse`
        self.client.call::<ListdescriptorsResponse>("listdescriptors", &params).await
    }

/// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
    pub async fn listlabels(&self, purpose: String) -> Result<ListlabelsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(purpose)?);
        // dispatch and deserialize to `ListlabelsResponse`
        self.client.call::<ListlabelsResponse>("listlabels", &params).await
    }

/// Returns list of temporarily unspendable outputs.
/// See the lockunspent call to lock and unlock transactions for spending.
    pub async fn listlockunspent(&self) -> Result<ListlockunspentResponse, TransportError> {
        // dispatch and deserialize to `ListlockunspentResponse`
        self.client.call::<ListlockunspentResponse>("listlockunspent", &[]).await
    }

/// List balances by receiving address.
    pub async fn listreceivedbyaddress(&self, minconf: u32, include_empty: bool, include_watchonly: bool, address_filter: String, include_immature_coinbase: bool) -> Result<ListreceivedbyaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(include_empty)?);
        params.push(serde_json::to_value(include_watchonly)?);
        params.push(serde_json::to_value(address_filter)?);
        params.push(serde_json::to_value(include_immature_coinbase)?);
        // dispatch and deserialize to `ListreceivedbyaddressResponse`
        self.client.call::<ListreceivedbyaddressResponse>("listreceivedbyaddress", &params).await
    }

/// List received transactions by label.
    pub async fn listreceivedbylabel(&self, minconf: u32, include_empty: bool, include_watchonly: bool, include_immature_coinbase: bool) -> Result<ListreceivedbylabelResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(include_empty)?);
        params.push(serde_json::to_value(include_watchonly)?);
        params.push(serde_json::to_value(include_immature_coinbase)?);
        // dispatch and deserialize to `ListreceivedbylabelResponse`
        self.client.call::<ListreceivedbylabelResponse>("listreceivedbylabel", &params).await
    }

/// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
/// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
    pub async fn listsinceblock(&self, blockhash: bitcoin::BlockHash, target_confirmations: u64, include_watchonly: bool, include_removed: bool, include_change: bool, label: String) -> Result<ListsinceblockResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(blockhash)?);
        params.push(serde_json::to_value(target_confirmations)?);
        params.push(serde_json::to_value(include_watchonly)?);
        params.push(serde_json::to_value(include_removed)?);
        params.push(serde_json::to_value(include_change)?);
        params.push(serde_json::to_value(label)?);
        // dispatch and deserialize to `ListsinceblockResponse`
        self.client.call::<ListsinceblockResponse>("listsinceblock", &params).await
    }

/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
///
/// Returns up to "count" most recent transactions skipping the first "from" transactions.
    pub async fn listtransactions(&self, label: String, count: u64, skip: u64, include_watchonly: bool) -> Result<ListtransactionsResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(label)?);
        params.push(serde_json::to_value(count)?);
        params.push(serde_json::to_value(skip)?);
        params.push(serde_json::to_value(include_watchonly)?);
        // dispatch and deserialize to `ListtransactionsResponse`
        self.client.call::<ListtransactionsResponse>("listtransactions", &params).await
    }

/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
    pub async fn listunspent(&self, minconf: u32, maxconf: u32, addresses: Vec<String>, include_unsafe: bool, query_options: serde_json::Value) -> Result<ListunspentResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(minconf)?);
        params.push(serde_json::to_value(maxconf)?);
        params.push(serde_json::to_value(addresses)?);
        params.push(serde_json::to_value(include_unsafe)?);
        params.push(serde_json::to_value(query_options)?);
        // dispatch and deserialize to `ListunspentResponse`
        self.client.call::<ListunspentResponse>("listunspent", &params).await
    }

/// Returns a list of wallets in the wallet directory.
    pub async fn listwalletdir(&self) -> Result<ListwalletdirResponse, TransportError> {
        // dispatch and deserialize to `ListwalletdirResponse`
        self.client.call::<ListwalletdirResponse>("listwalletdir", &[]).await
    }

/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use "getwalletinfo"
    pub async fn listwallets(&self) -> Result<ListwalletsResponse, TransportError> {
        // dispatch and deserialize to `ListwalletsResponse`
        self.client.call::<ListwalletsResponse>("listwallets", &[]).await
    }

/// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.
    pub async fn loadwallet(&self, filename: String, load_on_startup: bool) -> Result<LoadwalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(filename)?);
        params.push(serde_json::to_value(load_on_startup)?);
        // dispatch and deserialize to `LoadwalletResponse`
        self.client.call::<LoadwalletResponse>("loadwallet", &params).await
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
        let mut params = Vec::new();
        params.push(serde_json::to_value(unlock)?);
        params.push(serde_json::to_value(transactions)?);
        params.push(serde_json::to_value(persistent)?);
        // dispatch and deserialize to `LockunspentResponse`
        self.client.call::<LockunspentResponse>("lockunspent", &params).await
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
        let mut params = Vec::new();
        params.push(serde_json::to_value(wallet_name)?);
        params.push(serde_json::to_value(passphrase)?);
        // dispatch and deserialize to `MigratewalletResponse`
        self.client.call::<MigratewalletResponse>("migratewallet", &params).await
    }

/// Entirely clears and refills the keypool.
/// WARNING: On non-HD wallets, this will require a new backup immediately, to include the new keys.
/// When restoring a backup of an HD wallet created before the newkeypool command is run, funds received to
/// new addresses may not appear automatically. They have not been lost, but the wallet may not find them.
/// This can be fixed by running the newkeypool command on the backup and then rescanning, so the wallet
/// re-generates the required keys.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn newkeypool(&self) -> Result<(), TransportError> {
        // dispatch and deserialize to `()`
        self.client.call::<()>("newkeypool", &[]).await
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
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        params.push(serde_json::to_value(options)?);
        // dispatch and deserialize to `PsbtbumpfeeResponse`
        self.client.call::<PsbtbumpfeeResponse>("psbtbumpfee", &params).await
    }

/// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
    pub async fn removeprunedfunds(&self, txid: bitcoin::Txid) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(txid)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("removeprunedfunds", &params).await
    }

/// Rescan the local blockchain for wallet related transactions.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// The rescan is significantly faster when used on a descriptor wallet
/// and block filters are available (using startup option "-blockfilterindex=1").
    pub async fn rescanblockchain(&self, start_height: u64, stop_height: u64) -> Result<RescanblockchainResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(start_height)?);
        params.push(serde_json::to_value(stop_height)?);
        // dispatch and deserialize to `RescanblockchainResponse`
        self.client.call::<RescanblockchainResponse>("rescanblockchain", &params).await
    }

/// Restores and loads a wallet from backup.
///
/// The rescan is significantly faster if a descriptor wallet is restored
/// and block filters are available (using startup option "-blockfilterindex=1").
    pub async fn restorewallet(&self, wallet_name: String, backup_file: String, load_on_startup: bool) -> Result<RestorewalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(wallet_name)?);
        params.push(serde_json::to_value(backup_file)?);
        params.push(serde_json::to_value(load_on_startup)?);
        // dispatch and deserialize to `RestorewalletResponse`
        self.client.call::<RestorewalletResponse>("restorewallet", &params).await
    }

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Send a transaction.
    pub async fn send(&self, outputs: Vec<serde_json::Value>, conf_target: u64, estimate_mode: String, fee_rate: f64, options: serde_json::Value) -> Result<SendResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(outputs)?);
        params.push(serde_json::to_value(conf_target)?);
        params.push(serde_json::to_value(estimate_mode)?);
        params.push(serde_json::to_value(fee_rate)?);
        params.push(serde_json::to_value(options)?);
        // dispatch and deserialize to `SendResponse`
        self.client.call::<SendResponse>("send", &params).await
    }

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
/// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using ``send_max`` to exclude inputs that are worth less than the fees needed to spend them.
    pub async fn sendall(&self, recipients: Vec<serde_json::Value>, conf_target: u64, estimate_mode: String, fee_rate: f64, options: serde_json::Value) -> Result<SendallResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(recipients)?);
        params.push(serde_json::to_value(conf_target)?);
        params.push(serde_json::to_value(estimate_mode)?);
        params.push(serde_json::to_value(fee_rate)?);
        params.push(serde_json::to_value(options)?);
        // dispatch and deserialize to `SendallResponse`
        self.client.call::<SendallResponse>("sendall", &params).await
    }

/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sendmany(&self, dummy: Option<String>, amounts: serde_json::Value, minconf: u32, comment: String, subtractfeefrom: Vec<serde_json::Value>, replaceable: bool, conf_target: u64, estimate_mode: String, fee_rate: f64, verbose: bool) -> Result<SendmanyResponse, TransportError> {
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
        // dispatch and deserialize to `SendmanyResponse`
        self.client.call::<SendmanyResponse>("sendmany", &params).await
    }

/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sendtoaddress(&self, address: String, amount: bitcoin::Amount, comment: String, comment_to: String, subtractfeefromamount: bool, replaceable: bool, conf_target: u64, estimate_mode: String, avoid_reuse: bool, fee_rate: f64, verbose: bool) -> Result<SendtoaddressResponse, TransportError> {
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
        // dispatch and deserialize to `SendtoaddressResponse`
        self.client.call::<SendtoaddressResponse>("sendtoaddress", &params).await
    }

/// Set or generate a new HD wallet seed. Non-HD wallets will not be upgraded to being a HD wallet. Wallets that are already
/// HD will have a new HD seed set so that new keys added to the keypool will be derived from this new seed.
///
/// Note that you will need to MAKE A NEW BACKUP of your wallet after setting the HD wallet seed.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
/// Note: This command is only compatible with legacy wallets.
    pub async fn sethdseed(&self, newkeypool: bool, seed: String) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(newkeypool)?);
        params.push(serde_json::to_value(seed)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("sethdseed", &params).await
    }

/// Sets the label associated with the given address.
    pub async fn setlabel(&self, address: String, label: String) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(label)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("setlabel", &params).await
    }

/// Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
/// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
    pub async fn settxfee(&self, amount: bitcoin::Amount) -> Result<SettxfeeResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(amount)?);
        // dispatch and deserialize to `SettxfeeResponse`
        self.client.call::<SettxfeeResponse>("settxfee", &params).await
    }

/// Change the state of the given wallet flag for a wallet.
    pub async fn setwalletflag(&self, flag: String, value: bool) -> Result<SetwalletflagResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(flag)?);
        params.push(serde_json::to_value(value)?);
        // dispatch and deserialize to `SetwalletflagResponse`
        self.client.call::<SetwalletflagResponse>("setwalletflag", &params).await
    }

/// Sign a message with the private key of an address
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn signmessage(&self, address: String, message: String) -> Result<SignmessageResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        params.push(serde_json::to_value(message)?);
        // dispatch and deserialize to `SignmessageResponse`
        self.client.call::<SignmessageResponse>("signmessage", &params).await
    }

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn signrawtransactionwithwallet(&self, hexstring: String, prevtxs: Vec<serde_json::Value>, sighashtype: String) -> Result<SignrawtransactionwithwalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(hexstring)?);
        params.push(serde_json::to_value(prevtxs)?);
        params.push(serde_json::to_value(sighashtype)?);
        // dispatch and deserialize to `SignrawtransactionwithwalletResponse`
        self.client.call::<SignrawtransactionwithwalletResponse>("signrawtransactionwithwallet", &params).await
    }

/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
    pub async fn simulaterawtransaction(&self, rawtxs: Vec<serde_json::Value>, options: serde_json::Value) -> Result<SimulaterawtransactionResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(rawtxs)?);
        params.push(serde_json::to_value(options)?);
        // dispatch and deserialize to `SimulaterawtransactionResponse`
        self.client.call::<SimulaterawtransactionResponse>("simulaterawtransaction", &params).await
    }

/// Unloads the wallet referenced by the request endpoint, otherwise unloads the wallet specified in the argument.
/// Specifying the wallet name on a wallet endpoint is invalid.
    pub async fn unloadwallet(&self, wallet_name: String, load_on_startup: bool) -> Result<UnloadwalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(wallet_name)?);
        params.push(serde_json::to_value(load_on_startup)?);
        // dispatch and deserialize to `UnloadwalletResponse`
        self.client.call::<UnloadwalletResponse>("unloadwallet", &params).await
    }

/// Upgrade the wallet. Upgrades to the latest version if no version number is specified.
/// New keys may be generated and a new wallet backup will need to be made.
    pub async fn upgradewallet(&self, version: u32) -> Result<UpgradewalletResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(version)?);
        // dispatch and deserialize to `UpgradewalletResponse`
        self.client.call::<UpgradewalletResponse>("upgradewallet", &params).await
    }

/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
    pub async fn walletcreatefundedpsbt(&self, inputs: Vec<serde_json::Value>, outputs: Vec<serde_json::Value>, locktime: u32, options: serde_json::Value, bip32derivs: bool) -> Result<WalletcreatefundedpsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(inputs)?);
        params.push(serde_json::to_value(outputs)?);
        params.push(serde_json::to_value(locktime)?);
        params.push(serde_json::to_value(options)?);
        params.push(serde_json::to_value(bip32derivs)?);
        // dispatch and deserialize to `WalletcreatefundedpsbtResponse`
        self.client.call::<WalletcreatefundedpsbtResponse>("walletcreatefundedpsbt", &params).await
    }

/// Display address on an external signer for verification.
    pub async fn walletdisplayaddress(&self, address: String) -> Result<WalletdisplayaddressResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(address)?);
        // dispatch and deserialize to `WalletdisplayaddressResponse`
        self.client.call::<WalletdisplayaddressResponse>("walletdisplayaddress", &params).await
    }

/// Removes the wallet encryption key from memory, locking the wallet.
/// After calling this method, you will need to call walletpassphrase again
/// before being able to call any methods which require the wallet to be unlocked.
    pub async fn walletlock(&self) -> Result<(), TransportError> {
        // dispatch and deserialize to `()`
        self.client.call::<()>("walletlock", &[]).await
    }

/// Stores the wallet decryption key in memory for "timeout" seconds.
/// This is needed prior to performing transactions related to private keys such as sending bitcoins
///
/// Note:
/// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
/// time that overrides the old one.
    pub async fn walletpassphrase(&self, passphrase: String, timeout: u64) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(passphrase)?);
        params.push(serde_json::to_value(timeout)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("walletpassphrase", &params).await
    }

/// Changes the wallet passphrase from "oldpassphrase" to "newpassphrase".
    pub async fn walletpassphrasechange(&self, oldpassphrase: String, newpassphrase: String) -> Result<(), TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(oldpassphrase)?);
        params.push(serde_json::to_value(newpassphrase)?);
        // dispatch and deserialize to `()`
        self.client.call::<()>("walletpassphrasechange", &params).await
    }

/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn walletprocesspsbt(&self, psbt: String, sign: bool, sighashtype: String, bip32derivs: bool, finalize: bool) -> Result<WalletprocesspsbtResponse, TransportError> {
        let mut params = Vec::new();
        params.push(serde_json::to_value(psbt)?);
        params.push(serde_json::to_value(sign)?);
        params.push(serde_json::to_value(sighashtype)?);
        params.push(serde_json::to_value(bip32derivs)?);
        params.push(serde_json::to_value(finalize)?);
        // dispatch and deserialize to `WalletprocesspsbtResponse`
        self.client.call::<WalletprocesspsbtResponse>("walletprocesspsbt", &params).await
    }
}

