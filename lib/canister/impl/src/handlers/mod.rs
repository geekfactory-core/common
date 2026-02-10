use ic_agent::{CallHttpSettings, IcAgentRequest, QueryHttpSettings};
use ic_request::builder::{
    build_call_request, build_query_request, BuildRequestEnvironment, BuildRequestError,
    CanisterRequest, RequestSender,
};

pub mod ic_agent;
pub mod ic_request;

pub enum IcAgentRequestDefinition {
    Query {
        request: CanisterRequest,
        settings: QueryHttpSettings,
    },
    Call {
        request: CanisterRequest,
        settings: CallHttpSettings,
    },
}

pub async fn build_ic_agent_request(
    env: &dyn BuildRequestEnvironment,
    request_definition: IcAgentRequestDefinition,
    sender: RequestSender,
) -> Result<IcAgentRequest, BuildRequestError> {
    match request_definition {
        IcAgentRequestDefinition::Query { request, settings } => {
            build_query_request(env, request, sender)
                .await
                .map(|signed_query_request| IcAgentRequest::Query {
                    signed_query_request,
                    settings,
                })
        }
        IcAgentRequestDefinition::Call { request, settings } => {
            build_call_request(env, request, sender)
                .await
                .map(|signed_call_request| IcAgentRequest::Call {
                    signed_call_request,
                    settings,
                })
        }
    }
}
