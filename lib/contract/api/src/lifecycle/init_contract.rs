use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::SignedContractCertificate;

pub type Args = InitContractArgs;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InitContractArgs {
    pub root_public_key_raw: Vec<u8>,
    pub certificate: SignedContractCertificate,
    pub contract_activation_code_hash: Option<Vec<u8>>,
}
