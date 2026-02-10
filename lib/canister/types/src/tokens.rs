use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

pub type TokenE8s = u64;
pub type TransactionId = u64;

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum LedgerAccount {
    AccountIdentifier {
        slice: Vec<u8>,
    },
    Account {
        owner: Principal,
        subaccount: Option<Vec<u8>>,
    },
}

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct TransferInformation {
    pub block_index: u64,
    pub receiver_account_hex: String,
    pub amount: TokenE8s,
}
