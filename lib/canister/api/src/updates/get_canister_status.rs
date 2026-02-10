use candid::{CandidType, Principal};
use ic_cdk::management_canister::CanisterStatusResult;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub canister_id: Principal,
}
pub type Response = GetCanisterStatusResponse;

#[allow(clippy::large_enum_variant)]
#[derive(CandidType, Deserialize, Debug)]
pub enum GetCanisterStatusResponse {
    Ok(GetCanisterStatusResult),
    Err(GetCanisterStatusError),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct GetCanisterStatusResult {
    pub canister_status_response: CanisterStatusResult,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetCanisterStatusError {
    ManagementCallError { reason: String },
}

impl From<Result<GetCanisterStatusResult, GetCanisterStatusError>> for GetCanisterStatusResponse {
    fn from(result: Result<GetCanisterStatusResult, GetCanisterStatusError>) -> Self {
        match result {
            Ok(result) => GetCanisterStatusResponse::Ok(result),
            Err(err) => GetCanisterStatusResponse::Err(err),
        }
    }
}

impl From<GetCanisterStatusResponse> for Result<GetCanisterStatusResult, GetCanisterStatusError> {
    fn from(response: GetCanisterStatusResponse) -> Self {
        match response {
            GetCanisterStatusResponse::Ok(result) => Ok(result),
            GetCanisterStatusResponse::Err(err) => Err(err),
        }
    }
}
