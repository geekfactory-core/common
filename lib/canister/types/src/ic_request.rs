use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

pub type IcRequestId = Vec<u8>;
pub type IcSignedRequest = Vec<u8>;

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct QueryCanisterSignedRequest {
    pub canister_id: Principal,
    pub request_sign: IcSignedRequest,
}

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct CallCanisterSignedRequest {
    pub canister_id: Principal,
    pub request_id: IcRequestId,
    pub request_sign: IcSignedRequest,
    pub read_state_request_sign: IcSignedRequest,
}
