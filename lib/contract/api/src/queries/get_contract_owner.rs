use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {}
pub type Response = GetContractOwnerResponse;

#[derive(CandidType, Deserialize, Debug)]
pub enum GetContractOwnerResponse {
    Ok(GetContractOwnerResult),
    Err(GetContractOwnerError),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct GetContractOwnerResult {
    pub owner: Principal,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetContractOwnerError {
    ContractActivationNotRequired,
    ContractNotActivated,
}

impl From<Result<GetContractOwnerResult, GetContractOwnerError>> for GetContractOwnerResponse {
    fn from(result: Result<GetContractOwnerResult, GetContractOwnerError>) -> Self {
        match result {
            Ok(result) => GetContractOwnerResponse::Ok(result),
            Err(err) => GetContractOwnerResponse::Err(err),
        }
    }
}
