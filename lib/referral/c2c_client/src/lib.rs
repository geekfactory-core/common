use candid::Principal;
use ic_cdk::call::CallResult;
use referral_canister_api::get_referral_reward_data::GetReferralRewardDataResponse;
pub use referral_canister_api::get_referral_reward_data::{
    GetReferralRewardDataArgs, GetReferralRewardDataError, GetReferralRewardDataResult,
};

pub async fn get_referral_reward_data(
    referral_canister: Principal,
    args: GetReferralRewardDataArgs,
) -> Result<GetReferralRewardDataResult, GetReferralRewardDataError> {
    call_referral_reward_data(referral_canister, args)
        .await
        .map_err(|err| GetReferralRewardDataError::CallError {
            reason: format!("{:?}", err),
        })
        .and_then(|response| response.into())
}

async fn call_referral_reward_data(
    referral_canister: Principal,
    args: GetReferralRewardDataArgs,
) -> CallResult<GetReferralRewardDataResponse> {
    Ok(ic_cdk::call::Call::bounded_wait(
        referral_canister,
        referral_canister_api::get_referral_reward_data::METHOD_NAME,
    )
    .with_arg(args)
    .await?
    .candid()?)
}
