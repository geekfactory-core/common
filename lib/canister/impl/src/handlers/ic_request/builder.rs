use crate::components::ecdsa::interface::EcdsaSignature;
use crate::components::rand::RandGenerator;
use crate::components::time::Time;
use crate::handlers::ic_request::request_id::{to_request_id, RequestId};
use crate::handlers::ic_request::sha256::get_sha256;
use crate::handlers::ic_request::types::{
    CallRequestContent, Envelope, IngressExpiryDatetimeNanos, QueryContent, ReadStateContent,
};
use candid::{CandidType, Principal};
use common_canister_types::{
    Asn1BlockPublicKey, CallCanisterSignedRequest, DerivationPath, EcdsaSignatureCompact,
    MessageHash, QueryCanisterSignedRequest,
};
use ic_certification::Label;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::rc::Rc;
use std::time::Duration;

use super::types::{DeviceKey, SignedDelegation};

pub trait BuildRequestEnvironment {
    fn get_time_(&self) -> Rc<dyn Time>;
    fn get_rand_generator(&self) -> Rc<dyn RandGenerator>;
    fn get_ecdsa_signature(&self) -> Rc<dyn EcdsaSignature>;
}

#[derive(Clone)]
pub enum RequestSender {
    Ecdsa {
        ecdsa_derivation_path: DerivationPath,
        device_key: DeviceKey,
    },
    EcdsaWithDelegation {
        ecdsa_derivation_path: DerivationPath,
        device_key: DeviceKey,
        delegation: SignedDelegation,
    },
}

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct CanisterRequest {
    pub canister_id: Principal,
    pub method_name: String,
    pub args: Vec<u8>,
}

#[derive(Debug)]
pub enum BuildRequestError {
    DelegationExpired,
    BuildError { reason: String },
}

// API

pub async fn build_query_request(
    env: &dyn BuildRequestEnvironment,
    canister_request: CanisterRequest,
    sender: RequestSender,
) -> Result<QueryCanisterSignedRequest, BuildRequestError> {
    let (derivation_path, device_key, delegation) = obtain_sender_information(env, sender)?;

    let sender = Principal::self_authenticating(&device_key);

    let ingress_expiry = get_ingress_expiry_datetime_nanos(env.get_time_().as_ref());

    // query request sign

    let request = QueryContent::QueryRequest {
        canister_id: canister_request.canister_id,
        method_name: canister_request.method_name,
        arg: canister_request.args,
        sender,
        ingress_expiry,
    };

    let request_id = to_request_id(&request).map_err(to_build_error)?;

    let message_hash = construct_sign_message_hash(&request_id);
    let sign_result = env
        .get_ecdsa_signature()
        .as_ref()
        .sign_with_ecdsa(derivation_path, &message_hash)
        .await
        .map_err(|reason| BuildRequestError::BuildError { reason })?;

    let request_sign = serialize_envelope(device_key, delegation, sign_result, &request)?;

    Ok(QueryCanisterSignedRequest {
        canister_id: canister_request.canister_id,
        request_sign,
    })
}

pub async fn build_call_request(
    env: &dyn BuildRequestEnvironment,
    canister_request: CanisterRequest,
    sender: RequestSender,
) -> Result<CallCanisterSignedRequest, BuildRequestError> {
    let (derivation_path, device_key, delegation) = obtain_sender_information(env, sender)?;

    let sender = Principal::self_authenticating(&device_key);

    let ingress_expiry = get_ingress_expiry_datetime_nanos(env.get_time_().as_ref());

    // call request sign

    let nonce = env
        .get_rand_generator()
        .as_ref()
        .generate_16()
        .await
        .map_err(to_build_error)?;

    let request = CallRequestContent::CallRequest {
        canister_id: canister_request.canister_id,
        method_name: canister_request.method_name,
        arg: canister_request.args,
        sender,
        nonce: Some(nonce),
        ingress_expiry,
    };

    let request_id = to_request_id(&request).map_err(to_build_error)?;

    let message_hash = construct_sign_message_hash(&request_id);
    let sign_result = env
        .get_ecdsa_signature()
        .as_ref()
        .sign_with_ecdsa(derivation_path.clone(), &message_hash)
        .await
        .map_err(to_build_error)?;

    let request_sign = serialize_envelope(
        device_key.clone(),
        delegation.clone(),
        sign_result,
        &request,
    )?;

    // read state request sign

    let rs_request = build_read_state_request(sender, &request_id, ingress_expiry);

    let rs_request_id = to_request_id(&rs_request).map_err(to_build_error)?;

    let rs_message_hash = construct_sign_message_hash(&rs_request_id);
    let rs_sign_result = env
        .get_ecdsa_signature()
        .as_ref()
        .sign_with_ecdsa(derivation_path, &rs_message_hash)
        .await
        .map_err(to_build_error)?;

    let read_state_request_sign =
        serialize_envelope(device_key, delegation, rs_sign_result, &rs_request)?;

    Ok(CallCanisterSignedRequest {
        canister_id: canister_request.canister_id,
        request_id: request_id.as_slice().to_vec(),
        request_sign,
        read_state_request_sign,
    })
}

pub fn get_ingress_expiry_datetime_nanos(time: &dyn Time) -> IngressExpiryDatetimeNanos {
    let permitted_drift = Duration::from_secs(60);
    let ingress_expiry_duration = Duration::from_secs(300);

    (ingress_expiry_duration
        .as_nanos()
        .saturating_add(time.get_current_unix_epoch_time_nanos())
        .saturating_sub(permitted_drift.as_nanos())) as u64
}

// PRIVATE

fn obtain_sender_information(
    env: &dyn BuildRequestEnvironment,
    sender: RequestSender,
) -> Result<(DerivationPath, DeviceKey, Option<Vec<SignedDelegation>>), BuildRequestError> {
    match sender {
        RequestSender::Ecdsa {
            ecdsa_derivation_path,
            device_key,
        } => Ok((ecdsa_derivation_path, device_key, None)),
        RequestSender::EcdsaWithDelegation {
            ecdsa_derivation_path,
            device_key,
            delegation,
        } => {
            if (delegation.delegation.expiration as u128)
                < env.get_time_().get_current_unix_epoch_time_nanos()
            {
                return Err(BuildRequestError::DelegationExpired);
            }

            Ok((ecdsa_derivation_path, device_key, Some(vec![delegation])))
        }
    }
}

fn build_read_state_request(
    sender: Principal,
    request_id: &RequestId,
    ingress_expiry: IngressExpiryDatetimeNanos,
) -> ReadStateContent {
    let paths: Vec<Vec<Label>> = vec![vec!["request_status".into(), request_id.to_vec().into()]];

    ReadStateContent::ReadStateRequest {
        sender,
        paths,
        ingress_expiry,
    }
}

fn construct_sign_message_hash(request_id: &RequestId) -> MessageHash {
    get_sha256(construct_message(request_id))
}

fn construct_message(request_id: &RequestId) -> Vec<u8> {
    const IC_REQUEST_DOMAIN_SEPARATOR: &[u8; 11] = b"\x0Aic-request";

    let mut buf = vec![];
    buf.extend_from_slice(IC_REQUEST_DOMAIN_SEPARATOR);
    buf.extend_from_slice(request_id.as_slice());
    buf
}

fn serialize_envelope<'a, V>(
    sender_pubkey: Asn1BlockPublicKey,
    sender_delegation: Option<Vec<SignedDelegation>>,
    signature: EcdsaSignatureCompact,
    request: &V,
) -> Result<Vec<u8>, BuildRequestError>
where
    V: 'a + Serialize,
{
    let envelope = Envelope {
        content: request,
        sender_pubkey: Some(sender_pubkey),
        sender_delegation,
        sender_sig: Some(signature),
    };

    let mut serialized_bytes = Vec::new();
    let mut serializer = serde_cbor::Serializer::new(&mut serialized_bytes);

    serializer.self_describe().map_err(to_build_error)?;
    envelope
        .serialize(&mut serializer)
        .map_err(to_build_error)?;

    Ok(serialized_bytes)
}

fn to_build_error<V: Display>(v: V) -> BuildRequestError {
    BuildRequestError::BuildError {
        reason: v.to_string(),
    }
}
