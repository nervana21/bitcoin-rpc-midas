//! Result structs for RPC method returns
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct AbandontransactionResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct AbortrescanResponse(pub bool);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct AddconnectionResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct AddnodeResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct AddpeeraddressResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct AnalyzepsbtResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct BackupwalletResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct BumpfeeResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ClearbannedResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct CombinepsbtResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct CombinerawtransactionResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ConverttopsbtResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct CreatemultisigResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct CreatepsbtResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct CreaterawtransactionResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct CreatewalletResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct CreatewalletdescriptorResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct DecodepsbtResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct DecoderawtransactionResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct DecodescriptResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct DescriptorprocesspsbtResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct DisconnectnodeResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct DumptxoutsetResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct EchoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct EchoipcResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct EchojsonResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct EncryptwalletResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct EnumeratesignersResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct EstimaterawfeeResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct EstimatesmartfeeResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct FinalizepsbtResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct FundrawtransactionResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GenerateblockResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GeneratetoaddressResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GeneratetodescriptorResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetaddednodeinfoResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetaddressesbylabelResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetaddressinfoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetaddrmaninfoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetbalanceResponse(pub f64);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetbalancesResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetbestblockhashResponse(pub bitcoin::BlockHash);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetblockchaininfoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetblockcountResponse(pub u64);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetblockfilterResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetblockfrompeerResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetblockhashResponse(pub bitcoin::BlockHash);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetblockstatsResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetchainstatesResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetchaintipsResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetchaintxstatsResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetconnectioncountResponse(pub u64);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetdeploymentinfoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetdescriptoractivityResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetdescriptorinfoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetdifficultyResponse(pub f64);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GethdkeysResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetindexinfoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetmempoolentryResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetmempoolinfoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetmininginfoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetnettotalsResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetnetworkhashpsResponse(pub u64);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetnetworkinfoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetnewaddressResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetnodeaddressesResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetpeerinfoResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetprioritisedtransactionsResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetrawaddrmanResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetrawchangeaddressResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetreceivedbyaddressResponse(pub f64);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetreceivedbylabelResponse(pub f64);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetrpcinfoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GettransactionResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GettxoutproofResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GettxoutsetinfoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GettxspendingprevoutResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetwalletinfoResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetzmqnotificationsResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ImportdescriptorsResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ImportmempoolResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ImportprunedfundsResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct InvalidateblockResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct JoinpsbtsResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct KeypoolrefillResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ListaddressgroupingsResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ListbannedResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ListdescriptorsResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ListlabelsResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ListlockunspentResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ListreceivedbyaddressResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ListreceivedbylabelResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ListsinceblockResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ListtransactionsResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ListunspentResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ListwalletdirResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ListwalletsResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct LoadtxoutsetResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct LoadwalletResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct LockunspentResponse(pub bool);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct LoggingResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct MigratewalletResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct MockschedulerResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct PingResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct PreciousblockResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct PrioritisetransactionResponse(pub bool);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct PruneblockchainResponse(pub u64);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct PsbtbumpfeeResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ReconsiderblockResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct RemoveprunedfundsResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct RescanblockchainResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct RestorewalletResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SavemempoolResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SchemaResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SendResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SendallResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SendmsgtopeerResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SendrawtransactionResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SetbanResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SetlabelResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SetmocktimeResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SetnetworkactiveResponse(pub bool);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SettxfeeResponse(pub bool);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SetwalletflagResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SignmessageResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SignmessagewithprivkeyResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SignrawtransactionwithkeyResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SignrawtransactionwithwalletResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SimulaterawtransactionResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct StopResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SubmitheaderResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SubmitpackageResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SyncwithvalidationinterfacequeueResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TestmempoolacceptResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct UnloadwalletResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct UptimeResponse(pub u64);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct UtxoupdatepsbtResponse(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ValidateaddressResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct VerifychainResponse(pub bool);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct VerifymessageResponse(pub bool);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct VerifytxoutproofResponse(pub Vec<serde_json::Value>);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct WaitforblockResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct WaitforblockheightResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct WaitfornewblockResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct WalletcreatefundedpsbtResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct WalletdisplayaddressResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct WalletlockResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct WalletpassphraseResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct WalletpassphrasechangeResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct WalletprocesspsbtResponse(pub serde_json::Value);

