use async_trait::async_trait;

use crate::handlers::IcAgentRequestDefinition;

use super::api::{ListNeuronsResponse, ManageNeuronRequest, ManageNeuronResponse};

#[async_trait]
pub trait Nns: Sync + Send {
    fn build_get_neuron_ids_request(&self) -> IcAgentRequestDefinition;
    fn decode_get_neuron_ids_response(&self, response_data: &[u8]) -> Result<Vec<u64>, String>;

    fn build_get_list_neurons_request(&self, neuron_ids: Vec<u64>) -> IcAgentRequestDefinition;
    fn decode_get_list_neurons_response(
        &self,
        response_data: &[u8],
    ) -> Result<ListNeuronsResponse, String>;

    fn build_manage_neuron_request(&self, arg: ManageNeuronRequest) -> IcAgentRequestDefinition;
    fn decode_manage_neuron_response(
        &self,
        response_data: &[u8],
    ) -> Result<ManageNeuronResponse, String>;
}
