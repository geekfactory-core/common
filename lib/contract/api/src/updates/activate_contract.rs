use candid::{CandidType, Principal};
use common_canister_types::DelayedTimestampMillis;
use serde::Deserialize;

use crate::ContractActivationCode;

pub type Args = ActivateContractArgs;
pub type Response = ActivateContractResponse;

pub const METHOD_NAME: &str = "activate_contract";

#[derive(CandidType, Deserialize, Debug)]
pub struct ActivateContractArgs {
    pub check_permission_strategy: CheckPermissionStrategy,
    pub contract_owner: Principal,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum CheckPermissionStrategy {
    CheckContractActivationCode { code: ContractActivationCode },
    CheckCallerIsDeployer,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ActivateContractResponse {
    Ok,
    Err(ActivateContractError),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ActivateContractError {
    ContractActivationNotRequired,
    ContractLocked { lock: DelayedTimestampMillis },
    ContractCallError { reason: String },
    AlreadyActivated { owner: Principal },
    ValidationFailed { reason: String },
}

impl From<Result<(), ActivateContractError>> for ActivateContractResponse {
    fn from(r: Result<(), ActivateContractError>) -> Self {
        match r {
            Ok(()) => ActivateContractResponse::Ok,
            Err(error) => ActivateContractResponse::Err(error),
        }
    }
}

impl From<ActivateContractResponse> for Result<(), ActivateContractError> {
    fn from(response: ActivateContractResponse) -> Self {
        match response {
            ActivateContractResponse::Ok => Ok(()),
            ActivateContractResponse::Err(error) => Err(error),
        }
    }
}
