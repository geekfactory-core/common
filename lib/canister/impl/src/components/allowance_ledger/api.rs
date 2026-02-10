use candid::CandidType;
use ic_ledger_types::Tokens;
use serde::{Deserialize, Serialize};

use crate::components::nns_dap::api::AccountIdentifier;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GetAllowancesArgs {
    pub from_account_id: AccountIdentifier,
    pub prev_spender_id: Option<AccountIdentifier>,
    pub take: Option<u64>,
}

/// The allowance returned by the `get_allowances` endpoint.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Allowance {
    pub from_account_id: AccountIdentifier,
    pub to_spender_id: AccountIdentifier,
    pub allowance: Tokens,
    pub expires_at: Option<u64>,
}

/// The allowances vector returned by the `get_allowances` endpoint.
pub type Allowances = Vec<Allowance>;
