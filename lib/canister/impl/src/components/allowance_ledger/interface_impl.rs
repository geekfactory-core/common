use async_trait::async_trait;
use candid::Principal;
use common_canister_types::LedgerAccount;
use ic_cdk::call::CallResult;
use icrc_ledger_types::icrc1::account::Account;

use crate::components::allowance_ledger::api::Allowances;
use crate::components::ledger::to_account_identifier;

use super::api::GetAllowancesArgs;
use super::interface::AllowanceLedger;

pub struct AllowanceLedgerImpl {
    ledger_canister_id: Principal,
}

impl AllowanceLedgerImpl {
    pub fn new(ledger_canister_id: Principal) -> Self {
        Self { ledger_canister_id }
    }
}

#[async_trait]
impl AllowanceLedger for AllowanceLedgerImpl {
    async fn get_allowances(
        &self,
        arg: icrc_ledger_types::icrc103::get_allowances::GetAllowancesArgs,
    ) -> CallResult<Allowances> {
        let arg_converted = GetAllowancesArgs {
            from_account_id: arg
                .from_account
                .as_ref()
                .map(from_account_to_account_identifier_hex)
                .unwrap(),
            prev_spender_id: arg
                .prev_spender
                .as_ref()
                .map(from_account_to_account_identifier_hex),
            take: arg.take.map(|v| v.0.to_u64_digits()[0]),
        };

        Ok(
            ic_cdk::call::Call::bounded_wait(self.ledger_canister_id, "get_allowances")
                .with_arg(arg_converted)
                .await?
                .candid()?,
        )
    }
}

pub fn from_account_to_account_identifier_hex(
    account: &Account,
) -> crate::components::nns_dap::api::AccountIdentifier {
    let ledger_account = LedgerAccount::Account {
        owner: account.owner,
        subaccount: account.subaccount.map(|s| s.to_vec()),
    };
    to_account_identifier(&ledger_account).unwrap().to_hex()
}
