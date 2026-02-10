use candid::{CandidType, Principal};
use common_canister_types::DelayedTimestampMillis;
use serde::Deserialize;

pub const METHOD_NAME: &str = "add_contract_controller";

pub type Args = AddContractControllerArgs;
pub type Response = AddContractControllerResponse;

#[derive(CandidType, Deserialize, Debug)]
pub struct AddContractControllerArgs {
    pub controller: Principal,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum AddContractControllerResponse {
    Ok,
    Err(AddContractControllerError),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum AddContractControllerError {
    ContractNotActivated,
    PermissionDenied,
    CertificateNotExpired,
    ContractLocked { lock: DelayedTimestampMillis },
    CriticalCyclesLevel { critical_threshold_cycles: u128 },
    AddControllerDelay { delay: DelayedTimestampMillis },
    ManagementCallError { reason: String },
}

impl From<Result<(), AddContractControllerError>> for AddContractControllerResponse {
    fn from(r: Result<(), AddContractControllerError>) -> Self {
        match r {
            Ok(()) => AddContractControllerResponse::Ok,
            Err(error) => AddContractControllerResponse::Err(error),
        }
    }
}

impl From<AddContractControllerResponse> for Result<(), AddContractControllerError> {
    fn from(response: AddContractControllerResponse) -> Self {
        match response {
            AddContractControllerResponse::Ok => Ok(()),
            AddContractControllerResponse::Err(error) => Err(error),
        }
    }
}
