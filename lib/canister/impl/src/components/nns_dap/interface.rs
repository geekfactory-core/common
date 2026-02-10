use async_trait::async_trait;

use crate::handlers::IcAgentRequestDefinition;

use super::api::GetAccountResponse;

#[async_trait]
pub trait NnsDapp: Sync + Send {
    fn build_get_account_request(&self) -> IcAgentRequestDefinition;
    fn decode_get_account_response(
        &self,
        response_data: &[u8],
    ) -> Result<GetAccountResponse, String>;
}
