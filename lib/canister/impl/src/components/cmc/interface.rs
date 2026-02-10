use async_trait::async_trait;
use candid::{CandidType, Principal};
use ic_ledger_types::{BlockIndex, Memo, Subaccount};
use serde::Deserialize;

use crate::components::cmc::api::IcpXdrConversionRate;

use super::api::{CreateCanisterArg, CreateCanisterError, NotifyError};

pub const MEMO_TOP_UP_CANISTER: Memo = Memo(0x50555054); // == 'TPUP'

#[derive(CandidType, Deserialize)]
pub enum CallWrapperError<E> {
    CallError { reason: String },
    WrappedError { error: E },
}

#[async_trait]
pub trait Cmc: Sync + Send {
    fn get_canister_sub_account(&self, canister: Principal) -> Subaccount {
        Subaccount::from(canister)
    }

    fn get_top_up_canister_memo(&self) -> Memo {
        MEMO_TOP_UP_CANISTER
    }

    async fn notify_top_up(
        &self,
        cmc_canister_id: Principal,
        block_index: BlockIndex,
        canister_id: Principal,
    ) -> Result<u128, CallWrapperError<NotifyError>>;

    async fn create_canister(
        &self,
        cmc_canister_id: Principal,
        arg: CreateCanisterArg,
        cycles: u128,
    ) -> Result<Principal, CallWrapperError<CreateCanisterError>>;

    async fn get_icp_xdr_conversion_rate(
        &self,
        cmc_canister_id: Principal,
    ) -> Result<IcpXdrConversionRate, CallWrapperError<()>>;
}
