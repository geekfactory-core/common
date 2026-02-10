use std::{future::Future, pin::Pin};

use candid::Principal;
use common_canister_types::{CallCanisterSignedRequest, QueryCanisterSignedRequest};
use executor::{call::execute_ic_call, query::execute_ic_query};
use ic_cdk::call::CallResult;
use serde::{Deserialize, Serialize};

pub mod executor;
pub mod sleeper;
pub mod types;
pub mod verify;

#[derive(Debug)]
pub enum IcAgentRequest {
    Query {
        signed_query_request: QueryCanisterSignedRequest,
        settings: QueryHttpSettings,
    },
    Call {
        signed_call_request: CallCanisterSignedRequest,
        settings: CallHttpSettings,
    },
}

#[derive(Debug, Default)]
pub struct QueryHttpSettings {
    pub max_response_bytes: Option<u64>,
}

#[derive(Debug, Default)]
pub struct CallHttpSettings {
    pub call_max_response_bytes: Option<u64>,
    pub poll_max_response_bytes: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TransformerCtx {
    Query,
    Call,
    CallStatus {
        effective_canister_id: Principal,
        request_id: Vec<u8>,
    },
}

// API

pub async fn execute_ic_agent_request(
    ic_url: String,
    sleeper: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>,
    request: IcAgentRequest,
    transform_canister_id: Principal,
    transform_query_method: String,
) -> CallResult<Vec<u8>> {
    match request {
        IcAgentRequest::Query {
            signed_query_request,
            settings,
        } => {
            execute_ic_query(
                ic_url,
                signed_query_request,
                transform_canister_id,
                transform_query_method,
                serde_cbor::to_vec(&TransformerCtx::Query).unwrap(),
                settings.max_response_bytes,
            )
            .await
        }
        IcAgentRequest::Call {
            signed_call_request,
            settings,
        } => {
            let poll_transformer_ctx = serde_cbor::to_vec(&TransformerCtx::CallStatus {
                effective_canister_id: signed_call_request.canister_id,
                request_id: signed_call_request.request_id.clone(),
            })
            .unwrap();

            execute_ic_call(
                ic_url,
                signed_call_request,
                settings.call_max_response_bytes,
                serde_cbor::to_vec(&TransformerCtx::Call).unwrap(),
                sleeper,
                settings.poll_max_response_bytes,
                poll_transformer_ctx,
                transform_canister_id,
                transform_query_method,
            )
            .await
        }
    }
}
