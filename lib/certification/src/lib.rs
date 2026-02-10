use common_contract_api::{ContractCertificate, SignedContractCertificate};
use ic_canister_sig_creation::{
    signature_map::{CanisterSigError, CanisterSigInputs, SignatureMap},
    CanisterSigPublicKey,
};
pub use ic_certification::{labeled_hash, Hash};
use ic_representation_independent_hash::{representation_independent_hash, Value};
use ic_signature_verification::verify_canister_sig;

pub const CERTIFICATE_SIG_DOMAIN: &[u8] = b"contract_certificate";
pub const LABEL_SIG: &[u8] = b"sig";

pub trait Certification: Sync + Send {
    fn add_contract_signature_to_signature_map(
        &self,
        sigs: &mut SignatureMap,
        contract_certificate: &ContractCertificate,
    ) -> Hash;

    fn get_signed_contract_certificate(
        &self,
        sigs: &SignatureMap,
        contract_certificate: &ContractCertificate,
    ) -> Result<SignedContractCertificate, CanisterSigError>;

    fn verify_signed_contract_certificate(
        &self,
        signed_contract_certificate: &SignedContractCertificate,
        ic_root_public_key_raw: &[u8],
    ) -> Result<(), String>;
}

pub struct CertificationImpl;

impl Certification for CertificationImpl {
    fn add_contract_signature_to_signature_map(
        &self,
        sigs: &mut SignatureMap,
        contract_certificate: &ContractCertificate,
    ) -> Hash {
        add_contract_signature_to_signature_map(sigs, contract_certificate)
    }

    fn get_signed_contract_certificate(
        &self,
        sigs: &SignatureMap,
        contract_certificate: &ContractCertificate,
    ) -> Result<SignedContractCertificate, CanisterSigError> {
        get_signed_contract_certificate(sigs, contract_certificate)
    }

    fn verify_signed_contract_certificate(
        &self,
        signed_contract_certificate: &SignedContractCertificate,
        ic_root_public_key_raw: &[u8],
    ) -> Result<(), String> {
        verify_signed_contract_certificate(signed_contract_certificate, ic_root_public_key_raw)
    }
}

pub fn add_contract_signature_to_signature_map(
    sigs: &mut SignatureMap,
    contract_certificate: &ContractCertificate,
) -> Hash {
    let inputs = CanisterSigInputs {
        domain: CERTIFICATE_SIG_DOMAIN,
        seed: contract_certificate.contract_canister.as_slice(),
        message: &get_contract_signature_msg(contract_certificate),
    };
    sigs.add_signature(&inputs);
    labeled_hash(LABEL_SIG, &sigs.root_hash())
}

pub fn get_contract_signature_msg(certificate: &ContractCertificate) -> Vec<u8> {
    let m: Vec<(String, Value)> = vec![
        (
            "hub_canister".into(),
            Value::Bytes(certificate.hub_canister.as_slice().to_vec()),
        ),
        (
            "deployer".into(),
            Value::Bytes(certificate.deployer.as_slice().to_vec()),
        ),
        (
            "contract_template_id".into(),
            Value::Bytes(certificate.contract_template_id.to_be_bytes().to_vec()),
        ),
        (
            "contract_wasm_hash".into(),
            Value::Bytes(certificate.contract_wasm_hash.as_bytes().to_vec()),
        ),
        ("expiration".into(), Value::Number(certificate.expiration)),
    ];
    representation_independent_hash(m.as_slice()).to_vec()
}

pub fn get_signed_contract_certificate(
    sigs: &SignatureMap,
    contract_certificate: &ContractCertificate,
) -> Result<SignedContractCertificate, CanisterSigError> {
    let inputs = CanisterSigInputs {
        domain: CERTIFICATE_SIG_DOMAIN,
        seed: contract_certificate.contract_canister.as_slice(),
        message: &get_contract_signature_msg(contract_certificate),
    };

    sigs.get_signature_as_cbor(&inputs, None)
        .map(|signature| SignedContractCertificate {
            contract_certificate: contract_certificate.clone(),
            signature,
        })
}

pub fn verify_signed_contract_certificate(
    signed_contract_certificate: &SignedContractCertificate,
    ic_root_public_key_raw: &[u8],
) -> Result<(), String> {
    let public_key_der = CanisterSigPublicKey {
        canister_id: signed_contract_certificate
            .contract_certificate
            .hub_canister,
        seed: signed_contract_certificate
            .contract_certificate
            .contract_canister
            .as_slice()
            .to_vec(),
    }
    .to_der();

    let mut msg = vec![];
    msg.extend([CERTIFICATE_SIG_DOMAIN.len() as u8]);
    msg.extend(CERTIFICATE_SIG_DOMAIN.to_vec());
    msg.extend(&get_contract_signature_msg(
        &signed_contract_certificate.contract_certificate,
    ));

    verify_canister_sig(
        &msg,
        signed_contract_certificate.signature.as_slice(),
        public_key_der.as_slice(),
        ic_root_public_key_raw,
    )
}
