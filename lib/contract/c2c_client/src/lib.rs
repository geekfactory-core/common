use candid::Principal;
pub use common_contract_api::{
    activate_contract::{ActivateContractArgs, ActivateContractError, ActivateContractResponse},
    get_contract_certificate::{
        GetContractCertificateError, GetContractCertificateResponse, GetContractCertificateResult,
    },
    ContractActivationCode,
};
use ic_cdk::call::CallResult;
use std::fmt::Debug;

pub async fn activate_contract(
    contract_canister: Principal,
    args: ActivateContractArgs,
) -> Result<(), ActivateContractError> {
    call_activate_contract(contract_canister, args)
        .await
        .map_err(call_error_to_reason)
        .map_err(|reason| ActivateContractError::ContractCallError { reason })
        .and_then(|response| response.into())
}

async fn call_activate_contract(
    contract_canister: Principal,
    args: ActivateContractArgs,
) -> CallResult<ActivateContractResponse> {
    Ok(ic_cdk::call::Call::bounded_wait(
        contract_canister,
        common_contract_api::activate_contract::METHOD_NAME,
    )
    .with_arg(args)
    .await?
    .candid()?)
}

pub async fn get_contract_certificate(
    contract_canister: Principal,
) -> Result<GetContractCertificateResult, GetContractCertificateError> {
    call_get_contract_certificate(contract_canister)
        .await
        .map_err(call_error_to_reason)
        .map_err(|reason| GetContractCertificateError::ContractCallError { reason })
        .and_then(|response| response.into())
}

async fn call_get_contract_certificate(
    contract_canister: Principal,
) -> CallResult<GetContractCertificateResponse> {
    Ok(ic_cdk::call::Call::bounded_wait(
        contract_canister,
        common_contract_api::get_contract_certificate::METHOD_NAME,
    )
    .with_arg(common_contract_api::get_contract_certificate::Args {})
    .await?
    .candid()?)
}

fn call_error_to_reason<E: Debug>(error: E) -> String {
    format!("contract call error: {error:?}")
}
