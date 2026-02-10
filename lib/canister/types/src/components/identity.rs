use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct OpenIdCredentialKey(pub Iss, pub Sub);

pub type Timestamp = u64;

pub type Iss = String;
pub type Sub = String;
