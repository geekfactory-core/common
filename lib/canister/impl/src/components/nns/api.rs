use candid::{self, CandidType, Deserialize, Principal};

pub const NNS_GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";

#[derive(CandidType, Deserialize)]
pub struct NeuronId {
    pub id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct Followees {
    pub followees: Vec<NeuronId>,
}

#[derive(CandidType, Deserialize)]
pub struct KnownNeuronData {
    pub name: String,
    pub description: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct RefreshVotingPower {}

#[derive(CandidType, Deserialize)]
pub struct RemoveHotKey {
    pub hot_key_to_remove: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub enum Operation {
    RemoveHotKey(RemoveHotKey),
    //     AddHotKey(AddHotKey),
    //     ChangeAutoStakeMaturity(ChangeAutoStakeMaturity),
    //     StopDissolving {},
    //     StartDissolving {},
    //     IncreaseDissolveDelay(IncreaseDissolveDelay),
    //     SetVisibility(SetVisibility),
    //     JoinCommunityFund {},
    //     LeaveCommunityFund {},
    //     SetDissolveTimestamp(SetDissolveTimestamp),
}

#[derive(CandidType, Deserialize)]
pub struct Configure {
    pub operation: Option<Operation>,
}

#[derive(CandidType, Deserialize)]
pub struct ProposalId {
    pub id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct NeuronStakeTransfer {
    pub to_subaccount: serde_bytes::ByteBuf,
    pub neuron_stake_e8s: u64,
    pub from: Option<Principal>,
    pub memo: u64,
    pub from_subaccount: serde_bytes::ByteBuf,
    pub transfer_timestamp: u64,
    pub block_height: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct GovernanceError {
    pub error_message: String,
    pub error_type: i32,
}

#[derive(CandidType, Deserialize)]
pub struct BallotInfo {
    pub vote: i32,
    pub proposal_id: Option<ProposalId>,
}

#[derive(CandidType, Deserialize)]
pub enum DissolveState {
    DissolveDelaySeconds(u64),
    WhenDissolvedTimestampSeconds(u64),
}

#[derive(CandidType, Deserialize)]
pub struct Neuron {
    pub id: Option<NeuronId>,
    pub staked_maturity_e8s_equivalent: Option<u64>,
    pub controller: Option<Principal>,
    pub recent_ballots: Vec<BallotInfo>,
    pub voting_power_refreshed_timestamp_seconds: Option<u64>,
    pub kyc_verified: bool,
    pub potential_voting_power: Option<u64>,
    pub neuron_type: Option<i32>,
    pub not_for_profit: bool,
    pub maturity_e8s_equivalent: u64,
    pub deciding_voting_power: Option<u64>,
    pub cached_neuron_stake_e8s: u64,
    pub created_timestamp_seconds: u64,
    pub auto_stake_maturity: Option<bool>,
    pub aging_since_timestamp_seconds: u64,
    pub hot_keys: Vec<Principal>,
    pub account: serde_bytes::ByteBuf,
    pub joined_community_fund_timestamp_seconds: Option<u64>,
    pub dissolve_state: Option<DissolveState>,
    pub followees: Vec<(i32, Followees)>,
    pub neuron_fees_e8s: u64,
    pub visibility: Option<i32>,
    pub transfer: Option<NeuronStakeTransfer>,
    pub known_neuron_data: Option<KnownNeuronData>,
    pub spawn_at_timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct NeuronInfo {
    pub dissolve_delay_seconds: u64,
    pub recent_ballots: Vec<BallotInfo>,
    pub voting_power_refreshed_timestamp_seconds: Option<u64>,
    pub potential_voting_power: Option<u64>,
    pub neuron_type: Option<i32>,
    pub deciding_voting_power: Option<u64>,
    pub created_timestamp_seconds: u64,
    pub state: i32,
    pub stake_e8s: u64,
    pub joined_community_fund_timestamp_seconds: Option<u64>,
    pub retrieved_at_timestamp_seconds: u64,
    pub visibility: Option<i32>,
    pub known_neuron_data: Option<KnownNeuronData>,
    pub voting_power: u64,
    pub age_seconds: u64,
}

#[derive(CandidType, Deserialize)]
pub struct NeuronSubaccount {
    pub subaccount: serde_bytes::ByteBuf,
}

#[derive(CandidType, Deserialize)]
pub struct ListNeurons {
    pub page_size: Option<u64>,
    pub include_public_neurons_in_full_neurons: Option<bool>,
    pub neuron_ids: Vec<u64>,
    pub page_number: Option<u64>,
    pub include_empty_neurons_readable_by_caller: Option<bool>,
    pub neuron_subaccounts: Option<Vec<NeuronSubaccount>>,
    pub include_neurons_readable_by_caller: bool,
}

#[derive(CandidType, Deserialize)]
pub struct ListNeuronsResponse {
    pub neuron_infos: Vec<(u64, NeuronInfo)>,
    pub full_neurons: Vec<Neuron>,
    pub total_pages_available: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub enum ManageNeuronCommandRequest {
    // Spawn(Spawn),
    // Split(Split),
    // Follow(Follow),
    RefreshVotingPower(RefreshVotingPower),
    // ClaimOrRefresh(ClaimOrRefresh),
    Configure(Configure),
    // RegisterVote(RegisterVote),
    // Merge(Merge),
    // DisburseToNeuron(DisburseToNeuron),
    // MakeProposal(MakeProposalRequest),
    // StakeMaturity(StakeMaturity),
    // MergeMaturity(MergeMaturity),
    // Disburse(Disburse),
}

#[derive(CandidType, Deserialize)]
pub struct ManageNeuronRequest {
    pub id: Option<NeuronId>,
    pub command: Option<ManageNeuronCommandRequest>,
    // pub neuron_id_or_subaccount: Option<NeuronIdOrSubaccount>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct RefreshVotingPowerResponse {}

#[derive(CandidType, Deserialize, Debug)]
pub enum Command1 {
    Error(GovernanceError),
    // Spawn(SpawnResponse),
    // Split(SpawnResponse),
    //     Follow {},
    RefreshVotingPower(RefreshVotingPowerResponse),
    //     ClaimOrRefresh(ClaimOrRefreshResponse),
    Configure {},
    //     RegisterVote {},
    //     Merge(MergeResponse),
    //     DisburseToNeuron(SpawnResponse),
    //     MakeProposal(MakeProposalResponse),
    //     StakeMaturity(StakeMaturityResponse),
    //     MergeMaturity(MergeMaturityResponse),
    //     Disburse(DisburseResponse),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ManageNeuronResponse {
    pub command: Option<Command1>,
}
