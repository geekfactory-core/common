use super::execute_ic_request;
use crate::handlers::ic_agent::executor::deserialize_cbor_data;
use crate::handlers::ic_agent::types::{ReadStateResponse, RejectResponse};
use candid::Principal;
use common_canister_types::CallCanisterSignedRequest;
use ic_cdk::call::{CallRejected, CallResult, Error};
use ic_cdk::management_canister::HttpMethod;
use ic_certification::{Certificate, Label, LookupResult};
use std::future::Future;
use std::pin::Pin;
use std::str::from_utf8;

#[allow(clippy::too_many_arguments)]
pub async fn execute_ic_call(
    ic_url: String,
    request: CallCanisterSignedRequest,
    call_max_response_bytes: Option<u64>,
    call_transformer_ctx: Vec<u8>,
    sleeper: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>,
    poll_max_response_bytes: Option<u64>,
    poll_transformer_ctx: Vec<u8>,
    transform_canister_id: Principal,
    transform_method: String,
) -> CallResult<Vec<u8>> {
    let effective_canister_id = request.canister_id;

    execute_ic_request(
        ic_url.clone(),
        HttpMethod::POST,
        &format!("api/v2/canister/{effective_canister_id}/call"),
        Some(request.request_sign),
        transform_canister_id,
        transform_method.clone(),
        call_transformer_ctx,
        call_max_response_bytes,
    )
    .await
    .and_then(|http_response| {
        let status: u16 = http_response.status.to_string().parse().unwrap();
        if status == 200 {
            let cbor_decoded_body: Result<RejectResponse, serde_cbor::Error> =
                serde_cbor::from_slice(&http_response.body);

            Err(match cbor_decoded_body {
                Ok(replica_error) => Error::CallRejected(CallRejected::with_rejection(
                    0u32,
                    format!("call after transform get replica error: {replica_error:?}"),
                )),
                Err(cbor_error) => Error::CallRejected(CallRejected::with_rejection(
                    0u32,
                    format!("call after transform get cbor error: {cbor_error:?}"),
                )),
            })
        } else {
            Ok(http_response.body)
        }
    })?;

    sleeper().await;

    execute_ic_request(
        ic_url,
        HttpMethod::POST,
        &format!("api/v2/canister/{effective_canister_id}/read_state"),
        Some(request.read_state_request_sign),
        transform_canister_id,
        transform_method,
        poll_transformer_ctx,
        poll_max_response_bytes,
    )
    .await
    .and_then(|http_response| {
        if http_response.body.is_empty() {
            Err(Error::CallRejected(CallRejected::with_rejection(
                0u32,
                "read state body after transform is empty".to_owned(),
            )))
        } else {
            Ok(http_response.body)
        }
    })
}

pub fn get_certificate_from_state_response_body(
    response_body: &[u8],
) -> Result<Certificate, String> {
    let read_state_response: ReadStateResponse = deserialize_cbor_data(response_body)?;
    deserialize_cbor_data(&read_state_response.certificate)
}

pub fn get_reply_from_call_response_certificate(
    certificate: Certificate,
    request_id: &[u8],
) -> Result<Vec<u8>, String> {
    let path_status: [Label; 3] = ["request_status".into(), request_id.into(), "status".into()];

    if let LookupResult::Found(status) = certificate.tree.lookup_path(&path_status) {
        match from_utf8(status) {
            Ok("replied") => {
                let path_reply: [Label; 3] =
                    ["request_status".into(), request_id.into(), "reply".into()];

                if let LookupResult::Found(reply_data) = certificate.tree.lookup_path(&path_reply) {
                    Ok(Vec::from(reply_data))
                } else {
                    Err("No reply in certificate".to_string())
                }
            }
            _ => Err("Request status is not replied".to_string()),
        }
    } else {
        Err("No status in certificate".to_string())
    }
}
