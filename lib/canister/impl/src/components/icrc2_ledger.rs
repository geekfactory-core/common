use async_trait::async_trait;
use candid::{Nat, Principal};
use common_canister_types::LedgerAccount;
use ic_cdk::call::CallResult;
pub use icrc_ledger_types::icrc1::account::Account;
pub use icrc_ledger_types::icrc1::transfer::Memo;
pub use icrc_ledger_types::icrc2::{
    allowance::{Allowance, AllowanceArgs},
    approve::{ApproveArgs, ApproveError},
    transfer_from::{TransferFromArgs, TransferFromError},
};

use crate::components::ledger::vec_to_slice32;

pub type ApproveResult = Result<Nat, ApproveError>;
pub type TransferFromResult = Result<Nat, TransferFromError>;

#[async_trait]
pub trait ICRC2Ledger {
    async fn icrc2_approve(&self, arg: ApproveArgs) -> CallResult<ApproveResult>;

    async fn icrc2_allowance(&self, arg: AllowanceArgs) -> CallResult<Allowance>;

    async fn icrc2_transfer_from(&self, arg: TransferFromArgs) -> CallResult<TransferFromResult>;
}

pub struct ICRC2LedgerImpl {
    ledger_canister_id: Principal,
}

impl ICRC2LedgerImpl {
    pub fn new(ledger_canister_id: Principal) -> Self {
        Self { ledger_canister_id }
    }
}

#[async_trait]
impl ICRC2Ledger for ICRC2LedgerImpl {
    async fn icrc2_approve(&self, arg: ApproveArgs) -> CallResult<ApproveResult> {
        Ok(
            ic_cdk::call::Call::bounded_wait(self.ledger_canister_id, "icrc2_approve")
                .with_arg(arg)
                .await?
                .candid()?,
        )
    }

    async fn icrc2_allowance(&self, arg: AllowanceArgs) -> CallResult<Allowance> {
        Ok(
            ic_cdk::call::Call::bounded_wait(self.ledger_canister_id, "icrc2_allowance")
                .with_arg(arg)
                .await?
                .candid()?,
        )
    }

    async fn icrc2_transfer_from(&self, arg: TransferFromArgs) -> CallResult<TransferFromResult> {
        Ok(
            ic_cdk::call::Call::bounded_wait(self.ledger_canister_id, "icrc2_transfer_from")
                .with_arg(arg)
                .await?
                .candid()?,
        )
    }
}

pub fn to_icrc1_account(ledger_account: &LedgerAccount) -> Result<Account, String> {
    match ledger_account {
        LedgerAccount::AccountIdentifier { .. } => {
            Err("can not obtain account from slice".to_owned())
        }
        LedgerAccount::Account { owner, subaccount } => Ok(Account {
            owner: *owner,
            subaccount: subaccount.as_ref().map(vec_to_slice32).transpose()?,
        }),
    }
}
