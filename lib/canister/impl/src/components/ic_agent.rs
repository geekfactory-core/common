use std::{future::Future, pin::Pin};

use async_trait::async_trait;
use candid::Principal;
use ic_cdk::call::CallResult;

use crate::{
    components::time::Time,
    handlers::ic_agent::{self, IcAgentRequest},
};

#[async_trait(?Send)]
pub trait IcAgent {
    async fn execute_ic_agent_request(
        &self,
        ic_url: String,
        sleeper: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>,
        request: IcAgentRequest,
        transform_canister_id: Principal,
        transform_query_method: String,
    ) -> CallResult<Vec<u8>>;
}

pub struct IcAgentImpl {
    pub time: Pin<Box<dyn Time>>,
}

#[async_trait(?Send)]
impl IcAgent for IcAgentImpl {
    async fn execute_ic_agent_request(
        &self,
        ic_url: String,
        sleeper: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>,
        request: IcAgentRequest,
        transform_canister_id: Principal,
        transform_query_method: String,
    ) -> CallResult<Vec<u8>> {
        ic_agent::execute_ic_agent_request(
            ic_url,
            sleeper,
            request,
            transform_canister_id,
            transform_query_method,
        )
        .await
    }
}
