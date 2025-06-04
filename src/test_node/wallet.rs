use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
use crate::transport::core::TransportExt;
use crate::transport::{DefaultTransport, TransportError};

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
    pub async fn abandontransaction(&self, txid: bitcoin::Txid) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(txid)?);
        Ok(self.client.call::<Value>("abandontransaction", &vec).await?.into())
    }

/// Stops current wallet rescan triggered by an RPC call, e.g. by an importprivkey call.
/// Note: Use "getwalletinfo" to query the scanning progress.
    pub async fn abortrescan(&self) -> Result<Value, TransportError> {
        Ok(self.client.call::<Value>("abortrescan", &[]).await?.into())
    }

/// Add an nrequired-to-sign multisignature address to the wallet. Requires a new wallet backup.
/// Each key is a Bitcoin address or hex-encoded public key.
/// This functionality is only intended for use with non-watchonly addresses.
/// See ``importaddress`` for watchonly p2sh address support.
/// If "label" is specified, assign address to that label.
/// Note: This command is only compatible with legacy wallets.
    pub async fn addmultisigaddress(&self, nrequired: u64, keys: Vec<serde_json::Value>, label: String, address_type: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(nrequired)?);
        vec.push(serde_json::to_value(keys)?);
        vec.push(serde_json::to_value(label)?);
        vec.push(serde_json::to_value(address_type)?);
        Ok(self.client.call::<Value>("addmultisigaddress", &vec).await?.into())
    }

/// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
    pub async fn backupwallet(&self, destination: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(destination)?);
        Ok(self.client.call::<Value>("backupwallet", &vec).await?.into())
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
    pub async fn bumpfee(&self, txid: bitcoin::Txid, options: serde_json::Value) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(txid)?);
        vec.push(serde_json::to_value(options)?);
        Ok(self.client.call::<Value>("bumpfee", &vec).await?.into())
    }

/// Creates and loads a new wallet.
    pub async fn createwallet(&self, wallet_name: String, disable_private_keys: bool, blank: bool, passphrase: String, avoid_reuse: bool, descriptors: bool, load_on_startup: bool, external_signer: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(wallet_name)?);
        vec.push(serde_json::to_value(disable_private_keys)?);
        vec.push(serde_json::to_value(blank)?);
        vec.push(serde_json::to_value(passphrase)?);
        vec.push(serde_json::to_value(avoid_reuse)?);
        vec.push(serde_json::to_value(descriptors)?);
        vec.push(serde_json::to_value(load_on_startup)?);
        vec.push(serde_json::to_value(external_signer)?);
        Ok(self.client.call::<Value>("createwallet", &vec).await?.into())
    }

/// Reveals the private key corresponding to "address".
/// Then the importprivkey can be used with this output
/// Note: This command is only compatible with legacy wallets.
    pub async fn dumpprivkey(&self, address: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(address)?);
        Ok(self.client.call::<Value>("dumpprivkey", &vec).await?.into())
    }

/// Dumps all wallet keys in a human-readable format to a server-side file. This does not allow overwriting existing files.
/// Imported scripts are included in the dumpfile, but corresponding BIP173 addresses, etc. may not be added automatically by importwallet.
/// Note that if your wallet contains keys which are not derived from your HD seed (e.g. imported keys), these are not covered by
/// only backing up the seed itself, and must be backed up too (e.g. ensure you back up the whole dumpfile).
/// Note: This command is only compatible with legacy wallets.
    pub async fn dumpwallet(&self, filename: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(filename)?);
        Ok(self.client.call::<Value>("dumpwallet", &vec).await?.into())
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
    pub async fn encryptwallet(&self, passphrase: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(passphrase)?);
        Ok(self.client.call::<Value>("encryptwallet", &vec).await?.into())
    }

/// Returns the list of addresses assigned the specified label.
    pub async fn getaddressesbylabel(&self, label: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(label)?);
        Ok(self.client.call::<Value>("getaddressesbylabel", &vec).await?.into())
    }

/// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
    pub async fn getaddressinfo(&self, address: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(address)?);
        Ok(self.client.call::<Value>("getaddressinfo", &vec).await?.into())
    }

/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
    pub async fn getbalance(&self, dummy: String, minconf: u64, include_watchonly: bool, avoid_reuse: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(dummy)?);
        vec.push(serde_json::to_value(minconf)?);
        vec.push(serde_json::to_value(include_watchonly)?);
        vec.push(serde_json::to_value(avoid_reuse)?);
        Ok(self.client.call::<Value>("getbalance", &vec).await?.into())
    }

/// Returns an object with all balances in BTC.
    pub async fn getbalances(&self) -> Result<Value, TransportError> {
        Ok(self.client.call::<Value>("getbalances", &[]).await?.into())
    }

/// Returns a new Bitcoin address for receiving payments.
/// If "label" is specified, it is added to the address book
/// so payments received with the address will be associated with "label".
    pub async fn getnewaddress(&self, label: String, address_type: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(label)?);
        vec.push(serde_json::to_value(address_type)?);
        Ok(self.client.call::<Value>("getnewaddress", &vec).await?.into())
    }

/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
    pub async fn getrawchangeaddress(&self, address_type: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(address_type)?);
        Ok(self.client.call::<Value>("getrawchangeaddress", &vec).await?.into())
    }

/// Returns the total amount received by the given address in transactions with at least minconf confirmations.
    pub async fn getreceivedbyaddress(&self, address: String, minconf: u64, include_immature_coinbase: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(address)?);
        vec.push(serde_json::to_value(minconf)?);
        vec.push(serde_json::to_value(include_immature_coinbase)?);
        Ok(self.client.call::<Value>("getreceivedbyaddress", &vec).await?.into())
    }

/// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
    pub async fn getreceivedbylabel(&self, label: String, minconf: u64, include_immature_coinbase: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(label)?);
        vec.push(serde_json::to_value(minconf)?);
        vec.push(serde_json::to_value(include_immature_coinbase)?);
        Ok(self.client.call::<Value>("getreceivedbylabel", &vec).await?.into())
    }

/// Get detailed information about in-wallet transaction <txid>
    pub async fn gettransaction(&self, txid: String, include_watchonly: bool, verbose: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(txid)?);
        vec.push(serde_json::to_value(include_watchonly)?);
        vec.push(serde_json::to_value(verbose)?);
        Ok(self.client.call::<Value>("gettransaction", &vec).await?.into())
    }

/// DEPRECATED
/// Identical to getbalances().mine.untrusted_pending
    pub async fn getunconfirmedbalance(&self) -> Result<Value, TransportError> {
        Ok(self.client.call::<Value>("getunconfirmedbalance", &[]).await?.into())
    }

/// Returns an object containing various wallet state info.
    pub async fn getwalletinfo(&self) -> Result<Value, TransportError> {
        Ok(self.client.call::<Value>("getwalletinfo", &[]).await?.into())
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
    pub async fn importaddress(&self, address: String, label: String, rescan: bool, p2sh: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(address)?);
        vec.push(serde_json::to_value(label)?);
        vec.push(serde_json::to_value(rescan)?);
        vec.push(serde_json::to_value(p2sh)?);
        Ok(self.client.call::<Value>("importaddress", &vec).await?.into())
    }

/// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
/// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second elements will be imported as an internal descriptor.
///
/// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
    pub async fn importdescriptors(&self, requests: Vec<serde_json::Value>) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(requests)?);
        Ok(self.client.call::<Value>("importdescriptors", &vec).await?.into())
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
    pub async fn importmulti(&self, requests: Vec<serde_json::Value>, options: serde_json::Value) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(requests)?);
        vec.push(serde_json::to_value(options)?);
        Ok(self.client.call::<Value>("importmulti", &vec).await?.into())
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
    pub async fn importprivkey(&self, privkey: String, label: String, rescan: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(privkey)?);
        vec.push(serde_json::to_value(label)?);
        vec.push(serde_json::to_value(rescan)?);
        Ok(self.client.call::<Value>("importprivkey", &vec).await?.into())
    }

/// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
    pub async fn importprunedfunds(&self, rawtransaction: String, txoutproof: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(rawtransaction)?);
        vec.push(serde_json::to_value(txoutproof)?);
        Ok(self.client.call::<Value>("importprunedfunds", &vec).await?.into())
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
    pub async fn importpubkey(&self, pubkey: String, label: String, rescan: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(pubkey)?);
        vec.push(serde_json::to_value(label)?);
        vec.push(serde_json::to_value(rescan)?);
        Ok(self.client.call::<Value>("importpubkey", &vec).await?.into())
    }

/// Imports keys from a wallet dump file (see dumpwallet). Requires a new wallet backup to include imported keys.
/// Note: Blockchain and Mempool will be rescanned after a successful import. Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets.
    pub async fn importwallet(&self, filename: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(filename)?);
        Ok(self.client.call::<Value>("importwallet", &vec).await?.into())
    }

/// Fills the keypool.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn keypoolrefill(&self, newsize: u64) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(newsize)?);
        Ok(self.client.call::<Value>("keypoolrefill", &vec).await?.into())
    }

/// Lists groups of addresses which have had their common ownership
/// made public by common use as inputs or as the resulting change
/// in past transactions
    pub async fn listaddressgroupings(&self) -> Result<Value, TransportError> {
        Ok(self.client.call::<Value>("listaddressgroupings", &[]).await?.into())
    }

/// List descriptors imported into a descriptor-enabled wallet.
    pub async fn listdescriptors(&self, private: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(private)?);
        Ok(self.client.call::<Value>("listdescriptors", &vec).await?.into())
    }

/// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
    pub async fn listlabels(&self, purpose: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(purpose)?);
        Ok(self.client.call::<Value>("listlabels", &vec).await?.into())
    }

/// Returns list of temporarily unspendable outputs.
/// See the lockunspent call to lock and unlock transactions for spending.
    pub async fn listlockunspent(&self) -> Result<Value, TransportError> {
        Ok(self.client.call::<Value>("listlockunspent", &[]).await?.into())
    }

/// List balances by receiving address.
    pub async fn listreceivedbyaddress(&self, minconf: u64, include_empty: bool, include_watchonly: bool, address_filter: String, include_immature_coinbase: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(minconf)?);
        vec.push(serde_json::to_value(include_empty)?);
        vec.push(serde_json::to_value(include_watchonly)?);
        vec.push(serde_json::to_value(address_filter)?);
        vec.push(serde_json::to_value(include_immature_coinbase)?);
        Ok(self.client.call::<Value>("listreceivedbyaddress", &vec).await?.into())
    }

/// List received transactions by label.
    pub async fn listreceivedbylabel(&self, minconf: u64, include_empty: bool, include_watchonly: bool, include_immature_coinbase: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(minconf)?);
        vec.push(serde_json::to_value(include_empty)?);
        vec.push(serde_json::to_value(include_watchonly)?);
        vec.push(serde_json::to_value(include_immature_coinbase)?);
        Ok(self.client.call::<Value>("listreceivedbylabel", &vec).await?.into())
    }

/// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
/// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
    pub async fn listsinceblock(&self, blockhash: String, target_confirmations: u64, include_watchonly: bool, include_removed: bool, include_change: bool, label: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(blockhash)?);
        vec.push(serde_json::to_value(target_confirmations)?);
        vec.push(serde_json::to_value(include_watchonly)?);
        vec.push(serde_json::to_value(include_removed)?);
        vec.push(serde_json::to_value(include_change)?);
        vec.push(serde_json::to_value(label)?);
        Ok(self.client.call::<Value>("listsinceblock", &vec).await?.into())
    }

/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
///
/// Returns up to "count" most recent transactions skipping the first "from" transactions.
    pub async fn listtransactions(&self, label: String, count: u64, skip: u64, include_watchonly: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(label)?);
        vec.push(serde_json::to_value(count)?);
        vec.push(serde_json::to_value(skip)?);
        vec.push(serde_json::to_value(include_watchonly)?);
        Ok(self.client.call::<Value>("listtransactions", &vec).await?.into())
    }

/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
    pub async fn listunspent(&self, minconf: u64, maxconf: u64, addresses: Vec<bitcoin::Address<bitcoin::address::NetworkUnchecked>>, include_unsafe: bool, query_options: serde_json::Value) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(minconf)?);
        vec.push(serde_json::to_value(maxconf)?);
        vec.push(serde_json::to_value(addresses)?);
        vec.push(serde_json::to_value(include_unsafe)?);
        vec.push(serde_json::to_value(query_options)?);
        Ok(self.client.call::<Value>("listunspent", &vec).await?.into())
    }

/// Returns a list of wallets in the wallet directory.
    pub async fn listwalletdir(&self) -> Result<Value, TransportError> {
        Ok(self.client.call::<Value>("listwalletdir", &[]).await?.into())
    }

/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use "getwalletinfo"
    pub async fn listwallets(&self) -> Result<Value, TransportError> {
        Ok(self.client.call::<Value>("listwallets", &[]).await?.into())
    }

/// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.
    pub async fn loadwallet(&self, filename: String, load_on_startup: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(filename)?);
        vec.push(serde_json::to_value(load_on_startup)?);
        Ok(self.client.call::<Value>("loadwallet", &vec).await?.into())
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
    pub async fn lockunspent(&self, unlock: bool, transactions: Vec<serde_json::Value>, persistent: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(unlock)?);
        vec.push(serde_json::to_value(transactions)?);
        vec.push(serde_json::to_value(persistent)?);
        Ok(self.client.call::<Value>("lockunspent", &vec).await?.into())
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
    pub async fn migratewallet(&self, wallet_name: String, passphrase: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(wallet_name)?);
        vec.push(serde_json::to_value(passphrase)?);
        Ok(self.client.call::<Value>("migratewallet", &vec).await?.into())
    }

/// Entirely clears and refills the keypool.
/// WARNING: On non-HD wallets, this will require a new backup immediately, to include the new keys.
/// When restoring a backup of an HD wallet created before the newkeypool command is run, funds received to
/// new addresses may not appear automatically. They have not been lost, but the wallet may not find them.
/// This can be fixed by running the newkeypool command on the backup and then rescanning, so the wallet
/// re-generates the required keys.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn newkeypool(&self) -> Result<Value, TransportError> {
        Ok(self.client.call::<Value>("newkeypool", &[]).await?.into())
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
    pub async fn psbtbumpfee(&self, txid: bitcoin::Txid, options: serde_json::Value) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(txid)?);
        vec.push(serde_json::to_value(options)?);
        Ok(self.client.call::<Value>("psbtbumpfee", &vec).await?.into())
    }

/// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
    pub async fn removeprunedfunds(&self, txid: bitcoin::Txid) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(txid)?);
        Ok(self.client.call::<Value>("removeprunedfunds", &vec).await?.into())
    }

/// Rescan the local blockchain for wallet related transactions.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// The rescan is significantly faster when used on a descriptor wallet
/// and block filters are available (using startup option "-blockfilterindex=1").
    pub async fn rescanblockchain(&self, start_height: u64, stop_height: u64) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(start_height)?);
        vec.push(serde_json::to_value(stop_height)?);
        Ok(self.client.call::<Value>("rescanblockchain", &vec).await?.into())
    }

/// Restores and loads a wallet from backup.
///
/// The rescan is significantly faster if a descriptor wallet is restored
/// and block filters are available (using startup option "-blockfilterindex=1").
    pub async fn restorewallet(&self, wallet_name: String, backup_file: String, load_on_startup: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(wallet_name)?);
        vec.push(serde_json::to_value(backup_file)?);
        vec.push(serde_json::to_value(load_on_startup)?);
        Ok(self.client.call::<Value>("restorewallet", &vec).await?.into())
    }

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Send a transaction.
    pub async fn send(&self, outputs: Vec<serde_json::Value>, conf_target: u64, estimate_mode: String, fee_rate: Option<bitcoin::Amount>, options: serde_json::Value) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(outputs)?);
        vec.push(serde_json::to_value(conf_target)?);
        vec.push(serde_json::to_value(estimate_mode)?);
        vec.push(match fee_rate { Some(v) => serde_json::to_value(v.to_btc())?, None => serde_json::Value::Null });
        vec.push(serde_json::to_value(options)?);
        Ok(self.client.call::<Value>("send", &vec).await?.into())
    }

/// EXPERIMENTAL warning: this call may be changed in future releases.
///
/// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
/// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using ``send_max`` to exclude inputs that are worth less than the fees needed to spend them.
    pub async fn sendall(&self, recipients: Vec<serde_json::Value>, conf_target: u64, estimate_mode: String, fee_rate: Option<bitcoin::Amount>, options: serde_json::Value) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(recipients)?);
        vec.push(serde_json::to_value(conf_target)?);
        vec.push(serde_json::to_value(estimate_mode)?);
        vec.push(match fee_rate { Some(v) => serde_json::to_value(v.to_btc())?, None => serde_json::Value::Null });
        vec.push(serde_json::to_value(options)?);
        Ok(self.client.call::<Value>("sendall", &vec).await?.into())
    }

/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sendmany(&self, dummy: String, amounts: serde_json::Value, minconf: u64, comment: String, subtractfeefrom: Vec<serde_json::Value>, replaceable: bool, conf_target: u64, estimate_mode: String, fee_rate: Option<bitcoin::Amount>, verbose: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(dummy)?);
        vec.push(serde_json::to_value(amounts)?);
        vec.push(serde_json::to_value(minconf)?);
        vec.push(serde_json::to_value(comment)?);
        vec.push(serde_json::to_value(subtractfeefrom)?);
        vec.push(serde_json::to_value(replaceable)?);
        vec.push(serde_json::to_value(conf_target)?);
        vec.push(serde_json::to_value(estimate_mode)?);
        vec.push(match fee_rate { Some(v) => serde_json::to_value(v.to_btc())?, None => serde_json::Value::Null });
        vec.push(serde_json::to_value(verbose)?);
        Ok(self.client.call::<Value>("sendmany", &vec).await?.into())
    }

/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sendtoaddress(&self, address: String, amount: bitcoin::Amount, comment: String, comment_to: String, subtractfeefromamount: bool, replaceable: bool, conf_target: u64, estimate_mode: String, avoid_reuse: bool, fee_rate: Option<bitcoin::Amount>, verbose: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(address)?);
        vec.push(serde_json::to_value(amount)?);
        vec.push(serde_json::to_value(comment)?);
        vec.push(serde_json::to_value(comment_to)?);
        vec.push(serde_json::to_value(subtractfeefromamount)?);
        vec.push(serde_json::to_value(replaceable)?);
        vec.push(serde_json::to_value(conf_target)?);
        vec.push(serde_json::to_value(estimate_mode)?);
        vec.push(serde_json::to_value(avoid_reuse)?);
        vec.push(match fee_rate { Some(v) => serde_json::to_value(v.to_btc())?, None => serde_json::Value::Null });
        vec.push(serde_json::to_value(verbose)?);
        Ok(self.client.call::<Value>("sendtoaddress", &vec).await?.into())
    }

/// Set or generate a new HD wallet seed. Non-HD wallets will not be upgraded to being a HD wallet. Wallets that are already
/// HD will have a new HD seed set so that new keys added to the keypool will be derived from this new seed.
///
/// Note that you will need to MAKE A NEW BACKUP of your wallet after setting the HD wallet seed.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
/// Note: This command is only compatible with legacy wallets.
    pub async fn sethdseed(&self, newkeypool: bool, seed: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(newkeypool)?);
        vec.push(serde_json::to_value(seed)?);
        Ok(self.client.call::<Value>("sethdseed", &vec).await?.into())
    }

/// Sets the label associated with the given address.
    pub async fn setlabel(&self, address: String, label: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(address)?);
        vec.push(serde_json::to_value(label)?);
        Ok(self.client.call::<Value>("setlabel", &vec).await?.into())
    }

/// Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
/// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
    pub async fn settxfee(&self, amount: bitcoin::Amount) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(amount)?);
        Ok(self.client.call::<Value>("settxfee", &vec).await?.into())
    }

/// Change the state of the given wallet flag for a wallet.
    pub async fn setwalletflag(&self, flag: String, value: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(flag)?);
        vec.push(serde_json::to_value(value)?);
        Ok(self.client.call::<Value>("setwalletflag", &vec).await?.into())
    }

/// Sign a message with the private key of an address
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn signmessage(&self, address: String, message: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(address)?);
        vec.push(serde_json::to_value(message)?);
        Ok(self.client.call::<Value>("signmessage", &vec).await?.into())
    }

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn signrawtransactionwithwallet(&self, hexstring: String, prevtxs: Vec<serde_json::Value>, sighashtype: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(hexstring)?);
        vec.push(serde_json::to_value(prevtxs)?);
        vec.push(serde_json::to_value(sighashtype)?);
        Ok(self.client.call::<Value>("signrawtransactionwithwallet", &vec).await?.into())
    }

/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
    pub async fn simulaterawtransaction(&self, rawtxs: Vec<serde_json::Value>, options: serde_json::Value) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(rawtxs)?);
        vec.push(serde_json::to_value(options)?);
        Ok(self.client.call::<Value>("simulaterawtransaction", &vec).await?.into())
    }

/// Unloads the wallet referenced by the request endpoint, otherwise unloads the wallet specified in the argument.
/// Specifying the wallet name on a wallet endpoint is invalid.
    pub async fn unloadwallet(&self, wallet_name: String, load_on_startup: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(wallet_name)?);
        vec.push(serde_json::to_value(load_on_startup)?);
        Ok(self.client.call::<Value>("unloadwallet", &vec).await?.into())
    }

/// Upgrade the wallet. Upgrades to the latest version if no version number is specified.
/// New keys may be generated and a new wallet backup will need to be made.
    pub async fn upgradewallet(&self, version: u64) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(version)?);
        Ok(self.client.call::<Value>("upgradewallet", &vec).await?.into())
    }

/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
    pub async fn walletcreatefundedpsbt(&self, inputs: Vec<serde_json::Value>, outputs: Vec<serde_json::Value>, locktime: u64, options: serde_json::Value, bip32derivs: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(inputs)?);
        vec.push(serde_json::to_value(outputs)?);
        vec.push(serde_json::to_value(locktime)?);
        vec.push(serde_json::to_value(options)?);
        vec.push(serde_json::to_value(bip32derivs)?);
        Ok(self.client.call::<Value>("walletcreatefundedpsbt", &vec).await?.into())
    }

/// Display address on an external signer for verification.
    pub async fn walletdisplayaddress(&self, address: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(address)?);
        Ok(self.client.call::<Value>("walletdisplayaddress", &vec).await?.into())
    }

/// Removes the wallet encryption key from memory, locking the wallet.
/// After calling this method, you will need to call walletpassphrase again
/// before being able to call any methods which require the wallet to be unlocked.
    pub async fn walletlock(&self) -> Result<Value, TransportError> {
        Ok(self.client.call::<Value>("walletlock", &[]).await?.into())
    }

/// Stores the wallet decryption key in memory for "timeout" seconds.
/// This is needed prior to performing transactions related to private keys such as sending bitcoins
///
/// Note:
/// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
/// time that overrides the old one.
    pub async fn walletpassphrase(&self, passphrase: String, timeout: u64) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(passphrase)?);
        vec.push(serde_json::to_value(timeout)?);
        Ok(self.client.call::<Value>("walletpassphrase", &vec).await?.into())
    }

/// Changes the wallet passphrase from "oldpassphrase" to "newpassphrase".
    pub async fn walletpassphrasechange(&self, oldpassphrase: String, newpassphrase: String) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(oldpassphrase)?);
        vec.push(serde_json::to_value(newpassphrase)?);
        Ok(self.client.call::<Value>("walletpassphrasechange", &vec).await?.into())
    }

/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn walletprocesspsbt(&self, psbt: String, sign: bool, sighashtype: String, bip32derivs: bool, finalize: bool) -> Result<Value, TransportError> {
        let mut vec = vec![];
        vec.push(serde_json::to_value(psbt)?);
        vec.push(serde_json::to_value(sign)?);
        vec.push(serde_json::to_value(sighashtype)?);
        vec.push(serde_json::to_value(bip32derivs)?);
        vec.push(serde_json::to_value(finalize)?);
        Ok(self.client.call::<Value>("walletprocesspsbt", &vec).await?.into())
    }
}

