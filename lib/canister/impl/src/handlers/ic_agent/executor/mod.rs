use candid::Principal;
use ic_cdk::{
    call::{CallRejected, CallResult, Error},
    management_canister::{
        http_request, HttpHeader, HttpMethod, HttpRequestArgs, HttpRequestResult, TransformContext,
        TransformFunc,
    },
};
use std::ops::Add;

pub mod call;
pub mod query;

#[allow(clippy::too_many_arguments)]
pub async fn execute_ic_request(
    ic_url: String,
    method: HttpMethod,
    endpoint: &str,
    body: Option<Vec<u8>>,
    transform_canister_id: Principal,
    transform_method: String,
    transformer_ctx: Vec<u8>,
    max_response_bytes: Option<u64>,
) -> CallResult<HttpRequestResult> {
    let url = ic_url.add(endpoint);

    let headers = vec![HttpHeader {
        name: "content-type".to_string(),
        value: "application/cbor".to_string(),
    }];

    let request = HttpRequestArgs {
        url,
        method,
        body,
        max_response_bytes,
        transform: Some(TransformContext {
            function: TransformFunc(candid::Func {
                principal: transform_canister_id,
                method: transform_method,
            }),
            context: transformer_ctx,
        }),
        headers,
        is_replicated: None,
    };

    // see:https://docs.rs/ic-cdk/latest/ic_cdk/api/management_canister/http_request/struct.HttpResponse.html
    // see https://internetcomputer.org/docs/current/references/ic-interface-spec#http-call
    //
    http_request(&request).await.and_then(|response| {
        let status: u16 = response.status.to_string().parse().unwrap();
        if status_is_client_error(status) || status_is_server_error(status) {
            Err(Error::CallRejected(CallRejected::with_rejection(
                status as u32,
                "client or server error".to_owned(),
            )))
        } else {
            Ok(response)
        }
    })
}

pub(crate) fn status_is_client_error(status: u16) -> bool {
    (400..500).contains(&status)
}

pub(crate) fn status_is_server_error(status: u16) -> bool {
    (500..600).contains(&status)
}

pub fn deserialize_cbor_data<A>(serialized_bytes: &[u8]) -> Result<A, String>
where
    A: serde::de::DeserializeOwned,
{
    serde_cbor::from_slice(serialized_bytes)
        .map_err(|error| format!("Failed to deserialize CBOR data: {error}"))
}
