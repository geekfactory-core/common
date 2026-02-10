/// Copied from https://dashboard.internetcomputer.org/canister/rkp4c-7iaaa-aaaaa-aaaca-cai
use candid::{CandidType, Principal};
use ic_cdk::{call::CallResult, management_canister::CanisterSettings};
use ic_ledger_types::BlockIndex;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct SubnetFilter {
    pub subnet_type: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub enum SubnetSelection {
    Filter(SubnetFilter),
    Subnet { subnet: Principal },
}

#[derive(CandidType, Deserialize)]
pub struct CreateCanisterArg {
    pub subnet_selection: Option<SubnetSelection>,
    pub settings: Option<CanisterSettings>,
    // pub subnet_type: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub enum CreateCanisterError {
    Refunded {
        create_error: String,
        refund_amount: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub enum CreateCanisterResult {
    Ok(Principal),
    Err(CreateCanisterError),
}

#[derive(CandidType, Deserialize)]
pub enum NotifyError {
    Refunded {
        block_index: Option<BlockIndex>,
        reason: String,
    },
    InvalidTransaction(String),
    Other {
        error_message: String,
        error_code: u64,
    },
    Processing,
    TransactionTooOld(BlockIndex),
}

#[derive(CandidType, Deserialize)]
pub struct NotifyTopUpArg {
    pub block_index: BlockIndex,
    pub canister_id: Principal,
}

pub type Cycles = candid::Nat;
#[derive(CandidType, Deserialize)]
pub enum NotifyTopUpResult {
    Ok(Cycles),
    Err(NotifyError),
}

#[derive(CandidType, Deserialize)]
pub struct IcpXdrConversionRateResponse {
    pub certificate: serde_bytes::ByteBuf,
    pub data: IcpXdrConversionRate,
    pub hash_tree: serde_bytes::ByteBuf,
}

#[derive(CandidType, Deserialize)]
pub struct IcpXdrConversionRate {
    pub xdr_permyriad_per_icp: u64,
    pub timestamp_seconds: u64,
}

pub struct Service(pub Principal);
impl Service {
    pub async fn create_canister(
        &self,
        arg0: CreateCanisterArg,
        cycles: u128,
    ) -> CallResult<CreateCanisterResult> {
        Ok(ic_cdk::call::Call::bounded_wait(self.0, "create_canister")
            .with_arg(arg0)
            .with_cycles(cycles)
            .await?
            .candid()?)
    }

    pub async fn notify_top_up(&self, arg0: NotifyTopUpArg) -> CallResult<NotifyTopUpResult> {
        Ok(ic_cdk::call::Call::bounded_wait(self.0, "notify_top_up")
            .with_arg(arg0)
            .await?
            .candid()?)
    }

    pub async fn get_icp_xdr_conversion_rate(&self) -> CallResult<IcpXdrConversionRateResponse> {
        Ok(
            ic_cdk::call::Call::bounded_wait(self.0, "get_icp_xdr_conversion_rate")
                .await?
                .candid()?,
        )
    }
}
