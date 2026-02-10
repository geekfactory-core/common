use async_trait::async_trait;
use candid::{Nat, Principal};
use common_canister_types::{LedgerAccount, TimestampNanos, TokenE8s};
use ic_ledger_types::{
    account_balance, transfer, AccountBalanceArgs, AccountIdentifier, Memo, Subaccount, Timestamp,
    Tokens, TransferArgs, TransferResult, DEFAULT_SUBACCOUNT,
};

#[async_trait]
pub trait Ledger {
    fn get_canister_account(&self, sub_account: &Subaccount) -> AccountIdentifier;

    async fn get_account_balance(&self, account: AccountIdentifier) -> Result<TokenE8s, String>;

    async fn get_canister_subaccount_balance(
        &self,
        sub_account: &Subaccount,
    ) -> Result<TokenE8s, String>;

    async fn transfer_from_canister(
        &self,
        memo: Memo,
        from: Subaccount,
        to: AccountIdentifier,
        amount: TokenE8s,
        fee: TokenE8s,
        created_at_time: Option<TimestampNanos>,
    ) -> Result<TransferResult, String>;

    async fn get_ledger_fee(&self) -> Result<TokenE8s, String>;
}

pub struct LedgerImpl {
    ledger_canister_id: Principal,
    canister: Principal,
}

impl LedgerImpl {
    pub fn new(ledger_canister_id: Principal, canister: Principal) -> Self {
        Self {
            ledger_canister_id,
            canister,
        }
    }
}

#[async_trait]
impl Ledger for LedgerImpl {
    fn get_canister_account(&self, sub_account: &Subaccount) -> AccountIdentifier {
        AccountIdentifier::new(&self.canister, sub_account)
    }

    async fn get_account_balance(&self, account: AccountIdentifier) -> Result<TokenE8s, String> {
        account_balance(self.ledger_canister_id, &AccountBalanceArgs { account })
            .await
            .map(|t| t.e8s())
            .map_err(|e| format!("Error while getting account balance, reason: {e:?}"))
    }

    async fn get_canister_subaccount_balance(
        &self,
        sub_account: &Subaccount,
    ) -> Result<TokenE8s, String> {
        let account = self.get_canister_account(sub_account);
        self.get_account_balance(account).await
    }

    async fn transfer_from_canister(
        &self,
        memo: Memo,
        from: Subaccount,
        to: AccountIdentifier,
        amount: TokenE8s,
        fee: TokenE8s,
        created_at_time: Option<TimestampNanos>,
    ) -> Result<TransferResult, String> {
        let args = TransferArgs {
            memo,
            amount: Tokens::from_e8s(amount),
            fee: Tokens::from_e8s(fee),
            from_subaccount: Some(from),
            to,
            created_at_time: created_at_time.map(|nanos| Timestamp {
                timestamp_nanos: nanos as u64,
            }),
        };

        transfer(self.ledger_canister_id, &args)
            .await
            .map_err(|e| format!("Error while performing transfer call, reason: {e:?}"))
    }

    async fn get_ledger_fee(&self) -> Result<TokenE8s, String> {
        let fee = ic_cdk::call::Call::bounded_wait(self.ledger_canister_id, "icrc1_fee")
            .await
            .map_err(|e| format!("Error while getting ledger fee, reason: {e:?}"))?
            .candid::<Nat>()
            .map_err(|e| format!("Error while decoding ledger fee, reason: {e:?}"))?;

        fee.0
            .try_into()
            .map_err(|e| format!("Error while converting ledger fee to u64, reason: {e:?}"))
    }
}

pub fn validate_ledger_account(ledger_account: &LedgerAccount) -> bool {
    to_account_identifier(ledger_account).is_ok()
}

pub fn to_account_identifier(ledger_account: &LedgerAccount) -> Result<AccountIdentifier, String> {
    match ledger_account {
        LedgerAccount::AccountIdentifier { slice } => AccountIdentifier::from_slice(slice)
            .map_err(|error| format!("can not parse account identifier from slice: {error:?}")),
        LedgerAccount::Account { owner, subaccount } => Ok(AccountIdentifier::new(
            owner,
            &build_sub_account(subaccount)?,
        )),
    }
}

pub fn to_sub_account(ledger_account: &LedgerAccount) -> Result<Subaccount, String> {
    match ledger_account {
        LedgerAccount::AccountIdentifier { .. } => {
            Err("can not get sub account from account identifier".to_owned())
        }
        LedgerAccount::Account { subaccount, .. } => build_sub_account(subaccount),
    }
}

pub fn build_sub_account(subaccount: &Option<Vec<u8>>) -> Result<Subaccount, String> {
    match subaccount {
        None => Ok(DEFAULT_SUBACCOUNT),
        Some(sub_account) => vec_to_slice32(sub_account).map(Subaccount),
    }
}

pub fn vec_to_slice32(vec: &Vec<u8>) -> Result<[u8; 32], String> {
    if vec.len() == 32 {
        let array: [u8; 32] = vec
            .as_slice()
            .try_into()
            .expect("Length already checked to be 32");
        Ok(array)
    } else {
        Err(format!("Invalid length: {}. Expected 32 bytes.", vec.len()))
    }
}
