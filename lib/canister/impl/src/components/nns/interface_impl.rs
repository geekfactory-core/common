use async_trait::async_trait;
use candid::{Decode, Encode, Principal};

use crate::{
    components::nns::api::ManageNeuronResponse,
    handlers::{
        ic_agent::{CallHttpSettings, QueryHttpSettings},
        ic_request::builder::CanisterRequest,
        IcAgentRequestDefinition,
    },
};

use super::{
    api::{ListNeurons, ListNeuronsResponse, ManageNeuronRequest},
    interface::Nns,
};

pub struct NnsImpl {
    canister_id: Principal,
}

impl Default for NnsImpl {
    fn default() -> Self {
        Self {
            canister_id: Principal::from_text(super::api::NNS_GOVERNANCE_CANISTER_ID).unwrap(),
        }
    }
}

impl NnsImpl {
    pub fn new(nns_canister_id: Principal) -> Self {
        Self {
            canister_id: nns_canister_id,
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

    fn build_call(
        &self,
        method_name: &str,
        args: Vec<u8>,
        settings: CallHttpSettings,
    ) -> IcAgentRequestDefinition {
        IcAgentRequestDefinition::Call {
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
impl Nns for NnsImpl {
    fn build_get_neuron_ids_request(&self) -> IcAgentRequestDefinition {
        self.build_query(
            "get_neuron_ids",
            Encode!(&()).unwrap(),
            QueryHttpSettings::default(),
        )
    }

    fn decode_get_neuron_ids_response(&self, response_data: &[u8]) -> Result<Vec<u64>, String> {
        Decode!(response_data, Vec<u64>).map_err(|error| format!("{error:?}"))
    }

    fn build_get_list_neurons_request(&self, neuron_ids: Vec<u64>) -> IcAgentRequestDefinition {
        let arg = ListNeurons {
            neuron_ids,
            include_neurons_readable_by_caller: false,
            page_size: None,
            include_public_neurons_in_full_neurons: None,
            page_number: None,
            include_empty_neurons_readable_by_caller: None,
            neuron_subaccounts: None,
        };

        self.build_call(
            "list_neurons",
            Encode!(&arg).unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_get_list_neurons_response(
        &self,
        response_data: &[u8],
    ) -> Result<ListNeuronsResponse, String> {
        Decode!(response_data, ListNeuronsResponse).map_err(|error| format!("{error:?}"))
    }

    fn build_manage_neuron_request(&self, arg: ManageNeuronRequest) -> IcAgentRequestDefinition {
        self.build_call(
            "manage_neuron",
            Encode!(&arg).unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_manage_neuron_response(
        &self,
        response_data: &[u8],
    ) -> Result<ManageNeuronResponse, String> {
        Decode!(response_data, ManageNeuronResponse).map_err(|error| format!("{error:?}"))
    }
}
