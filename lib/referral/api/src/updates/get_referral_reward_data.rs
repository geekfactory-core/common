use candid::{CandidType, Principal};
use common_contract_api::ContractTemplateId;
use serde::Deserialize;

pub const METHOD_NAME: &str = "get_referral_reward_data";

pub type Args = GetReferralRewardDataArgs;
pub type Response = GetReferralRewardDataResponse;

#[derive(CandidType, Deserialize, Debug)]
pub struct GetReferralRewardDataArgs {
    pub referral: String,
    pub hub_canister: Principal,
    pub deployer: Principal,
    pub contract_template_id: ContractTemplateId,
    pub contract_canister: Principal,
    pub contract_owner: Principal,
    pub from_account_hex: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetReferralRewardDataResponse {
    Ok(GetReferralRewardDataResult),
    Err(GetReferralRewardDataError),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct GetReferralRewardDataResult {
    pub account_hex: String,
    pub memo: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetReferralRewardDataError {
    CallError { reason: String },
    ReferralNotFound,
}

impl From<Result<GetReferralRewardDataResult, GetReferralRewardDataError>>
    for GetReferralRewardDataResponse
{
    fn from(result: Result<GetReferralRewardDataResult, GetReferralRewardDataError>) -> Self {
        match result {
            Ok(ok) => GetReferralRewardDataResponse::Ok(ok),
            Err(err) => GetReferralRewardDataResponse::Err(err),
        }
    }
}

impl From<GetReferralRewardDataResponse>
    for Result<GetReferralRewardDataResult, GetReferralRewardDataError>
{
    fn from(response: GetReferralRewardDataResponse) -> Self {
        match response {
            GetReferralRewardDataResponse::Ok(result) => Ok(result),
            GetReferralRewardDataResponse::Err(err) => Err(err),
        }
    }
}
