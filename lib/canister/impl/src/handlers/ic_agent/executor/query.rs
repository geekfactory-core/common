use crate::handlers::ic_agent::{executor::deserialize_cbor_data, types::QueryResponse};
use candid::Principal;
use common_canister_types::QueryCanisterSignedRequest;
use ic_cdk::{
    call::{CallRejected, CallResult, Error},
    management_canister::HttpMethod,
};

use super::execute_ic_request;

#[allow(clippy::too_many_arguments)]
pub async fn execute_ic_query(
    ic_url: String,
    request: QueryCanisterSignedRequest,
    transform_canister_id: Principal,
    transform_method: String,
    transformer_ctx: Vec<u8>,
    max_response_bytes: Option<u64>,
) -> CallResult<Vec<u8>> {
    let effective_canister_id = request.canister_id;
    let envelope = request.request_sign;

    execute_ic_request(
        ic_url,
        HttpMethod::POST,
        &format!("api/v2/canister/{effective_canister_id}/query"),
        Some(envelope),
        transform_canister_id,
        transform_method,
        transformer_ctx,
        max_response_bytes,
    )
    .await
    .and_then(|http_response| {
        if http_response.body.is_empty() {
            Err(Error::CallRejected(CallRejected::with_rejection(
                0u32,
                "query body after transform is empty".to_owned(),
            )))
        } else {
            // in transformer we extract query reply information
            Ok(http_response.body)
        }
    })
}

pub fn get_reply_from_query_response_body(response_body: &[u8]) -> Result<Vec<u8>, String> {
    deserialize_cbor_data(response_body).and_then(|query_response: QueryResponse| {
        match query_response {
            QueryResponse::Replied { reply } => Ok(reply.arg),
            QueryResponse::Rejected(reject_response) => {
                Err(format!("Rejected: {reject_response:?}"))
            }
        }
    })
}
