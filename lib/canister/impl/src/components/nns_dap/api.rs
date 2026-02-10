use candid::{self, CandidType, Deserialize, Principal};

pub const NNS_DAPP_CANISTER_ID: &str = "qoctq-giaaa-aaaaa-aaaea-cai";

pub type AccountIdentifier = String;

pub type SubAccount = serde_bytes::ByteBuf;
#[derive(CandidType, Deserialize, Debug)]
pub struct SubAccountDetails {
    pub name: String,
    pub sub_account: SubAccount,
    pub account_identifier: AccountIdentifier,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct HardwareWalletAccountDetails {
    pub principal: Principal,
    pub name: String,
    pub account_identifier: AccountIdentifier,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct AccountDetails {
    pub principal: Principal,
    pub account_identifier: AccountIdentifier,
    pub hardware_wallet_accounts: Vec<HardwareWalletAccountDetails>,
    pub sub_accounts: Vec<SubAccountDetails>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetAccountResponse {
    Ok(AccountDetails),
    AccountNotFound,
}
