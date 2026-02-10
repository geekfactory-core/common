use async_trait::async_trait;
use ic_cdk::call::CallResult;
use icrc_ledger_types::icrc103::get_allowances::GetAllowancesArgs;

use crate::components::allowance_ledger::api::Allowances;

#[async_trait]
pub trait AllowanceLedger {
    async fn get_allowances(&self, arg: GetAllowancesArgs) -> CallResult<Allowances>;
}
