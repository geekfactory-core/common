use candid::{CandidType, Principal};
use common_canister_types::TimestampMillis;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub type ContractTemplateId = u64;
pub type WasmHash = String;
pub type ContractActivationCode = String;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ContractCertificate {
    pub hub_canister: Principal,
    pub deployer: Principal,
    pub contract_template_id: ContractTemplateId,
    pub contract_canister: Principal,
    pub contract_wasm_hash: WasmHash,
    pub expiration: TimestampMillis,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SignedContractCertificate {
    pub contract_certificate: ContractCertificate,
    pub signature: Vec<u8>,
}

pub fn get_wasm_hash_to_vec(wasm: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(wasm);
    hasher.finalize().to_vec()
}

pub fn get_wasm_hash(wasm: &Vec<u8>) -> WasmHash {
    let mut hasher = Sha256::new();
    hasher.update(wasm);
    format!("{:x}", hasher.finalize())
}

pub fn get_contract_activation_code_hash(code: ContractActivationCode) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(code);
    hasher.finalize().to_vec()
}
