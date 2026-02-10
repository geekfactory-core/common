use async_trait::async_trait;
use candid::Principal;
use ic_ledger_types::BlockIndex;

use crate::components::cmc::api::IcpXdrConversionRate;

use super::{
    api::{
        CreateCanisterArg, CreateCanisterError, CreateCanisterResult, NotifyError, NotifyTopUpArg,
        NotifyTopUpResult, Service,
    },
    interface::{CallWrapperError, Cmc},
};

pub struct CmcImpl;

#[async_trait]
impl Cmc for CmcImpl {
    async fn notify_top_up(
        &self,
        cmc_canister_id: Principal,
        block_index: BlockIndex,
        canister_id: Principal,
    ) -> Result<u128, CallWrapperError<NotifyError>> {
        Service(cmc_canister_id)
            .notify_top_up(NotifyTopUpArg {
                block_index,
                canister_id,
            })
            .await
            .map_err(|call_error| CallWrapperError::CallError {
                reason: format!("{call_error:?}"),
            })
            .and_then(|result| match result {
                NotifyTopUpResult::Ok(cycles) => Ok(cycles.0.try_into().unwrap_or(0)),
                NotifyTopUpResult::Err(error) => Err(CallWrapperError::WrappedError { error }),
            })
    }

    async fn create_canister(
        &self,
        cmc_canister_id: Principal,
        arg: CreateCanisterArg,
        cycles: u128,
    ) -> Result<Principal, CallWrapperError<CreateCanisterError>> {
        Service(cmc_canister_id)
            .create_canister(arg, cycles)
            .await
            .map_err(|call_error| CallWrapperError::CallError {
                reason: format!("{call_error:?}"),
            })
            .and_then(|result| match result {
                CreateCanisterResult::Ok(principal) => Ok(principal),
                CreateCanisterResult::Err(error) => Err(CallWrapperError::WrappedError { error }),
            })
    }

    async fn get_icp_xdr_conversion_rate(
        &self,
        cmc_canister_id: Principal,
    ) -> Result<IcpXdrConversionRate, CallWrapperError<()>> {
        Service(cmc_canister_id)
            .get_icp_xdr_conversion_rate()
            .await
            .map_err(|call_error| CallWrapperError::CallError {
                reason: format!("{call_error:?}"),
            })
            .map(|result| result.data)
    }
}
