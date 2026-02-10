use async_trait::async_trait;
use candid::{Decode, Encode, Principal};

use crate::handlers::{
    ic_agent::QueryHttpSettings, ic_request::builder::CanisterRequest, IcAgentRequestDefinition,
};

use super::{api::GetAccountResponse, interface::NnsDapp};

pub struct NnsDappImpl {
    canister_id: Principal,
}

impl Default for NnsDappImpl {
    fn default() -> Self {
        Self {
            canister_id: Principal::from_text(super::api::NNS_DAPP_CANISTER_ID).unwrap(),
        }
    }
}

impl NnsDappImpl {
    pub fn new(nns_dapp_canister: Principal) -> Self {
        Self {
            canister_id: nns_dapp_canister,
        }
    }

    fn build_query(
        &self,
        method_name: &str,
        args: Vec<u8>,
        settings: QueryHttpSettings,
    ) -> IcAgentRequestDefinition {
        IcAgentRequestDefinition::Query {
            request: self.build_canister_request(method_name, args),
            settings,
        }
    }

    fn build_canister_request(&self, method_name: &str, args: Vec<u8>) -> CanisterRequest {
        CanisterRequest {
            canister_id: self.canister_id,
            method_name: method_name.to_owned(),
            args,
        }
    }
}

#[async_trait]
impl NnsDapp for NnsDappImpl {
    fn build_get_account_request(&self) -> IcAgentRequestDefinition {
        self.build_query(
            "get_account",
            Encode!(&()).unwrap(),
            QueryHttpSettings::default(),
        )
    }

    fn decode_get_account_response(
        &self,
        response_data: &[u8],
    ) -> Result<GetAccountResponse, String> {
        Decode!(response_data, GetAccountResponse).map_err(|error| format!("{error:?}"))
    }
}
