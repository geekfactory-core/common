use candid::CandidType;
use common_canister_types::CanisterMetrics;
use serde::Deserialize;

pub type Args = GetCanisterMetricsArgs;
pub type Response = GetCanisterMetricsResponse;

#[derive(CandidType, Deserialize, Debug)]
pub struct GetCanisterMetricsArgs {}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetCanisterMetricsResponse {
    Ok(GetCanisterMetricsResult),
    Err(GetCanisterMetricsError),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct GetCanisterMetricsResult {
    pub metrics: CanisterMetrics,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetCanisterMetricsError {
    PermissionDenied,
}

impl From<Result<GetCanisterMetricsResult, GetCanisterMetricsError>>
    for GetCanisterMetricsResponse
{
    fn from(r: Result<GetCanisterMetricsResult, GetCanisterMetricsError>) -> Self {
        match r {
            Ok(result) => GetCanisterMetricsResponse::Ok(result),
            Err(error) => GetCanisterMetricsResponse::Err(error),
        }
    }
}
