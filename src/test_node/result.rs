//! Result structs for RPC method returns
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AbandontransactionResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct AbortrescanResult(pub bool);

#[derive(Debug, Deserialize)]
pub struct AddconnectionResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct AddmultisigaddressResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct AddnodeResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct AddpeeraddressResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct AnalyzepsbtResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ApiResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct BackupwalletResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct BumpfeeResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ClearbannedResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct CombinepsbtResult(pub String);

#[derive(Debug, Deserialize)]
pub struct CombinerawtransactionResult(pub String);

#[derive(Debug, Deserialize)]
pub struct ConverttopsbtResult(pub String);

#[derive(Debug, Deserialize)]
pub struct CreatemultisigResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct CreatepsbtResult(pub String);

#[derive(Debug, Deserialize)]
pub struct CreaterawtransactionResult(pub String);

#[derive(Debug, Deserialize)]
pub struct CreatewalletResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct CreatewalletdescriptorResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct DecodepsbtResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct DecoderawtransactionResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct DecodescriptResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct DescriptorprocesspsbtResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct DisconnectnodeResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct DumpprivkeyResult(pub String);

#[derive(Debug, Deserialize)]
pub struct DumptxoutsetResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct DumpwalletResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct EchoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct EchoipcResult(pub String);

#[derive(Debug, Deserialize)]
pub struct EchojsonResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct EncryptwalletResult(pub String);

#[derive(Debug, Deserialize)]
pub struct EnumeratesignersResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct EstimaterawfeeResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct EstimatesmartfeeResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct FinalizepsbtResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct FundrawtransactionResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GenerateblockResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GeneratetoaddressResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct GeneratetodescriptorResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct GetaddednodeinfoResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct GetaddressesbylabelResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetaddressinfoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetaddrmaninfoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetbalanceResult(pub bitcoin::Amount);

#[derive(Debug, Deserialize)]
pub struct GetbalancesResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetbestblockhashResult(pub String);

#[derive(Debug, Deserialize)]
pub struct GetblockchaininfoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetblockcountResult(pub u64);

#[derive(Debug, Deserialize)]
pub struct GetblockfilterResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetblockfrompeerResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetblockhashResult(pub String);

#[derive(Debug, Deserialize)]
pub struct GetblockstatsResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetchainstatesResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetchaintipsResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct GetchaintxstatsResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetconnectioncountResult(pub u64);

#[derive(Debug, Deserialize)]
pub struct GetdeploymentinfoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetdescriptorinfoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetdifficultyResult(pub u64);

#[derive(Debug, Deserialize)]
pub struct GethdkeysResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct GetindexinfoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetmempoolentryResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetmempoolinfoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetmininginfoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetnettotalsResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetnetworkhashpsResult(pub u64);

#[derive(Debug, Deserialize)]
pub struct GetnetworkinfoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetnewaddressResult(pub String);

#[derive(Debug, Deserialize)]
pub struct GetnodeaddressesResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct GetpeerinfoResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct GetprioritisedtransactionsResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetrawaddrmanResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GetrawchangeaddressResult(pub String);

#[derive(Debug, Deserialize)]
pub struct GetreceivedbyaddressResult(pub bitcoin::Amount);

#[derive(Debug, Deserialize)]
pub struct GetreceivedbylabelResult(pub bitcoin::Amount);

#[derive(Debug, Deserialize)]
pub struct GetrpcinfoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GettransactionResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GettxoutproofResult(pub String);

#[derive(Debug, Deserialize)]
pub struct GettxoutsetinfoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct GettxspendingprevoutResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct GetunconfirmedbalanceResult(pub u64);

#[derive(Debug, Deserialize)]
pub struct GetwalletinfoResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ImportaddressResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ImportdescriptorsResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct ImportmempoolResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ImportmultiResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct ImportprivkeyResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ImportprunedfundsResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ImportpubkeyResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ImportwalletResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct InvalidateblockResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct JoinpsbtsResult(pub String);

#[derive(Debug, Deserialize)]
pub struct KeypoolrefillResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ListaddressgroupingsResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct ListbannedResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct ListdescriptorsResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ListlabelsResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct ListlockunspentResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct ListreceivedbyaddressResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct ListreceivedbylabelResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct ListsinceblockResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ListtransactionsResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct ListunspentResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct ListwalletdirResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ListwalletsResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct LoadtxoutsetResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct LoadwalletResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct LockunspentResult(pub bool);

#[derive(Debug, Deserialize)]
pub struct LoggingResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct MigratewalletResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct MockschedulerResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct NewkeypoolResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct PingResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct PreciousblockResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct PrioritisetransactionResult(pub bool);

#[derive(Debug, Deserialize)]
pub struct PruneblockchainResult(pub u64);

#[derive(Debug, Deserialize)]
pub struct PsbtbumpfeeResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct ReconsiderblockResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct RemoveprunedfundsResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct RescanblockchainResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct RestorewalletResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SavemempoolResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SendResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SendallResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SendmsgtopeerResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SendrawtransactionResult(pub String);

#[derive(Debug, Deserialize)]
pub struct SetbanResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SethdseedResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SetlabelResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SetmocktimeResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SetnetworkactiveResult(pub bool);

#[derive(Debug, Deserialize)]
pub struct SettxfeeResult(pub bool);

#[derive(Debug, Deserialize)]
pub struct SetwalletflagResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SignmessageResult(pub String);

#[derive(Debug, Deserialize)]
pub struct SignmessagewithprivkeyResult(pub String);

#[derive(Debug, Deserialize)]
pub struct SignrawtransactionwithkeyResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SignrawtransactionwithwalletResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SimulaterawtransactionResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct StopResult(pub String);

#[derive(Debug, Deserialize)]
pub struct SubmitheaderResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SubmitpackageResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct SyncwithvalidationinterfacequeueResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct TestmempoolacceptResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct UnloadwalletResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct UpgradewalletResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct UptimeResult(pub u64);

#[derive(Debug, Deserialize)]
pub struct UtxoupdatepsbtResult(pub String);

#[derive(Debug, Deserialize)]
pub struct ValidateaddressResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct VerifychainResult(pub bool);

#[derive(Debug, Deserialize)]
pub struct VerifymessageResult(pub bool);

#[derive(Debug, Deserialize)]
pub struct VerifytxoutproofResult(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
pub struct WaitforblockResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct WaitforblockheightResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct WaitfornewblockResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct WalletcreatefundedpsbtResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct WalletdisplayaddressResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct WalletlockResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct WalletpassphraseResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct WalletpassphrasechangeResult(pub serde_json::Value);

#[derive(Debug, Deserialize)]
pub struct WalletprocesspsbtResult(pub serde_json::Value);

