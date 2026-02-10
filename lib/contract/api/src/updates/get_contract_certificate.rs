use candid::CandidType;
use serde::Deserialize;

use crate::SignedContractCertificate;

pub const METHOD_NAME: &str = "get_contract_certificate";

pub type Args = GetContractCertificateArgs;
pub type Response = GetContractCertificateResponse;

#[derive(CandidType, Deserialize, Debug)]
pub struct GetContractCertificateArgs {}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetContractCertificateResponse {
    Ok(GetContractCertificateResult),
    Err(GetContractCertificateError),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct GetContractCertificateResult {
    pub certificate: SignedContractCertificate,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetContractCertificateError {
    ContractCallError { reason: String },
}

impl From<GetContractCertificateResponse>
    for Result<GetContractCertificateResult, GetContractCertificateError>
{
    fn from(response: GetContractCertificateResponse) -> Self {
        match response {
            GetContractCertificateResponse::Ok(result) => Ok(result),
            GetContractCertificateResponse::Err(err) => Err(err),
        }
    }
}
