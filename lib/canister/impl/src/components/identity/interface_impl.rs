use std::time::Duration;

use async_trait::async_trait;
use candid::{Decode, Encode, Principal};
use common_canister_types::{components::identity::OpenIdCredentialKey, TimestampNanos};
use ic_representation_independent_hash::{representation_independent_hash, Value};

use super::{
    api::{
        AccountNumber, AuthnMethodData, AuthnMethodRegisterRet,
        AuthnMethodRegistrationModeEnterRet, AuthnMethodRemoveRet, GetAccountsRet,
        GetDefaultAccountRet, GetDelegationResponse, IdentityAuthnInfoRet, IdentityInfoRet,
        IdentityNumber, OpenidCredentialRemoveRet, PrepareAccountDelegationRet, PublicKey,
        UserNumber,
    },
    interface::Identity,
};
use crate::components::identity::api::{AuthnMethodConfirmRet, RegistrationId};
use crate::{
    components::identity::api::AuthnMethodRegistrationModeExitRet,
    handlers::{
        ic_agent::CallHttpSettings, ic_request::builder::CanisterRequest, IcAgentRequestDefinition,
    },
};

pub const DELEGATION_SIG_DOMAIN: &[u8] = b"ic-request-auth-delegation";

pub struct IdentityImpl {
    service: super::api::Service,
}

impl Default for IdentityImpl {
    fn default() -> Self {
        Self {
            service: super::api::Service(
                Principal::from_text(super::api::INTERNET_IDENTITY_CANISTER_ID).unwrap(),
            ),
        }
    }
}
impl IdentityImpl {
    pub fn new(identity_canister: Principal) -> Self {
        Self {
            service: super::api::Service(identity_canister),
        }
    }

    fn build_call(
        &self,
        method_name: &str,
        args: Vec<u8>,
        settings: CallHttpSettings,
    ) -> IcAgentRequestDefinition {
        IcAgentRequestDefinition::Call {
            request: self.build_canister_request(method_name, args),
            settings,
        }
    }

    fn build_canister_request(&self, method_name: &str, args: Vec<u8>) -> CanisterRequest {
        CanisterRequest {
            canister_id: self.service.0,
            method_name: method_name.to_owned(),
            args,
        }
    }
}

#[async_trait]
impl Identity for IdentityImpl {
    async fn identity_authn_info(
        &self,
        identity_number: IdentityNumber,
    ) -> Result<IdentityAuthnInfoRet, String> {
        self.service
            .identity_authn_info(identity_number)
            .await
            .map_err(|e| format!("Failed to call identity_authn_info, reason: {:?}", e))
    }

    fn build_identity_info_request(
        &self,
        identity_number: &IdentityNumber,
    ) -> IcAgentRequestDefinition {
        self.build_call(
            "identity_info",
            Encode!(identity_number).unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_identity_info_response(
        &self,
        response_data: &[u8],
    ) -> Result<IdentityInfoRet, String> {
        Decode!(response_data, IdentityInfoRet).map_err(|error| format!("{error:?}"))
    }

    async fn authn_method_register(
        &self,
        identity_number: IdentityNumber,
        authn_method_data: AuthnMethodData,
    ) -> Result<AuthnMethodRegisterRet, String> {
        self.service
            .authn_method_register(identity_number, authn_method_data)
            .await
            .map_err(|e| format!("Failed to call authn_method_register, reason: {:?}", e))
    }

    fn build_authn_method_session_register(
        &self,
        identity_number: &IdentityNumber,
    ) -> IcAgentRequestDefinition {
        self.build_call(
            "authn_method_session_register",
            Encode!(identity_number).unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_authn_method_session_register(
        &self,
        response_data: &[u8],
    ) -> Result<AuthnMethodRegisterRet, String> {
        Decode!(response_data, AuthnMethodRegisterRet).map_err(|error| format!("{error:?}"))
    }

    fn build_authn_method_remove_request(
        &self,
        identity_number: &IdentityNumber,
        public_key: &PublicKey,
    ) -> IcAgentRequestDefinition {
        self.build_call(
            "authn_method_remove",
            Encode!(identity_number, public_key).unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_authn_method_remove_response(
        &self,
        response_data: &[u8],
    ) -> Result<AuthnMethodRemoveRet, String> {
        Decode!(response_data, AuthnMethodRemoveRet).map_err(|error| format!("{error:?}"))
    }

    fn build_authn_method_registration_mode_exit_request(
        &self,
        identity_number: &IdentityNumber,
        auth_method_data: &Option<AuthnMethodData>,
    ) -> IcAgentRequestDefinition {
        self.build_call(
            "authn_method_registration_mode_exit",
            Encode!(identity_number, auth_method_data).unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_authn_method_registration_mode_exit_response(
        &self,
        response_data: &[u8],
    ) -> Result<AuthnMethodRegistrationModeExitRet, String> {
        Decode!(response_data, AuthnMethodRegistrationModeExitRet)
            .map_err(|error| format!("{error:?}"))
    }

    fn build_openid_credential_remove_request(
        &self,
        identity_number: &IdentityNumber,
        key: &OpenIdCredentialKey,
    ) -> IcAgentRequestDefinition {
        self.build_call(
            "openid_credential_remove",
            Encode!(identity_number, key).unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_openid_credential_remove_response(
        &self,
        response_data: &[u8],
    ) -> Result<OpenidCredentialRemoveRet, String> {
        Decode!(response_data, OpenidCredentialRemoveRet).map_err(|error| format!("{error:?}"))
    }

    fn build_get_principal_request(
        &self,
        user_number: &UserNumber,
        frontend_hostname: &String,
    ) -> IcAgentRequestDefinition {
        self.build_call(
            "get_principal",
            Encode!(user_number, frontend_hostname).unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_get_principal_response(&self, response_data: &[u8]) -> Result<Principal, String> {
        Decode!(response_data, Principal).map_err(|error| format!("{error:?}"))
    }

    fn build_authn_method_registration_mode_enter_request(
        &self,
        identity_number: &IdentityNumber,
        registration_id: &Option<RegistrationId>,
    ) -> IcAgentRequestDefinition {
        self.build_call(
            "authn_method_registration_mode_enter",
            Encode!(identity_number, registration_id).unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_authn_method_registration_mode_enter_response(
        &self,
        response_data: &[u8],
    ) -> Result<AuthnMethodRegistrationModeEnterRet, String> {
        Decode!(response_data, AuthnMethodRegistrationModeEnterRet)
            .map_err(|error| format!("{error:?}"))
    }

    fn build_authn_method_confirm_request(
        &self,
        identity_number: &IdentityNumber,
        verification_code: String,
    ) -> IcAgentRequestDefinition {
        self.build_call(
            "authn_method_confirm",
            Encode!(identity_number, &verification_code).unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_authn_method_confirm_response(
        &self,
        response_data: &[u8],
    ) -> Result<AuthnMethodConfirmRet, String> {
        Decode!(response_data, AuthnMethodConfirmRet).map_err(|error| format!("{error:?}"))
    }

    fn build_prepare_delegation_request(
        &self,
        user_number: &UserNumber,
        frontend_hostname: String,
        session_key: Vec<u8>,
        delegation_duration: Duration,
    ) -> IcAgentRequestDefinition {
        self.build_call(
            "prepare_delegation",
            Encode!(
                user_number,
                &frontend_hostname,
                &session_key,
                &Some(delegation_duration.as_nanos() as u64)
            )
            .unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_prepare_delegation_response(
        &self,
        response_data: &[u8],
    ) -> Result<(Vec<u8>, TimestampNanos), String> {
        Decode!(
            response_data,
            Vec<u8>,
            common_canister_types::components::identity::Timestamp
        )
        .map(|(data, time)| (data, time as TimestampNanos))
        .map_err(|error| format!("{error:?}"))
    }

    fn build_prepare_account_delegation_request(
        &self,
        user_number: &UserNumber,
        frontend_hostname: String,
        account_number: Option<AccountNumber>,
        session_key: Vec<u8>,
        delegation_duration: Duration,
    ) -> IcAgentRequestDefinition {
        self.build_call(
            "prepare_account_delegation",
            Encode!(
                user_number,
                &frontend_hostname,
                &account_number,
                &session_key,
                &Some(delegation_duration.as_nanos() as u64)
            )
            .unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_prepare_account_delegation_response(
        &self,
        response_data: &[u8],
    ) -> Result<PrepareAccountDelegationRet, String> {
        Decode!(response_data, PrepareAccountDelegationRet).map_err(|error| format!("{error:?}"))
    }

    fn build_get_delegation_request(
        &self,
        user_number: &UserNumber,
        frontend_hostname: String,
        session_key: Vec<u8>,
        timestamp: TimestampNanos,
    ) -> CanisterRequest {
        self.build_canister_request(
            "get_delegation",
            Encode!(
                user_number,
                &frontend_hostname,
                &session_key,
                &(timestamp as u64)
            )
            .unwrap(),
        )
    }

    fn decode_get_delegation_response(
        &self,
        response_data: &[u8],
    ) -> Result<GetDelegationResponse, String> {
        Decode!(response_data, GetDelegationResponse).map_err(|error| format!("{error:?}"))
    }

    fn build_get_default_account_request(
        &self,
        user_number: &UserNumber,
        frontend_hostname: String,
    ) -> IcAgentRequestDefinition {
        self.build_call(
            "get_default_account",
            Encode!(user_number, &frontend_hostname).unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_get_default_account_response(
        &self,
        response_data: &[u8],
    ) -> Result<GetDefaultAccountRet, String> {
        Decode!(response_data, GetDefaultAccountRet).map_err(|error| format!("{error:?}"))
    }

    fn build_get_accounts_request(
        &self,
        user_number: &UserNumber,
        frontend_hostname: String,
    ) -> IcAgentRequestDefinition {
        self.build_call(
            "get_accounts",
            Encode!(user_number, &frontend_hostname).unwrap(),
            CallHttpSettings::default(),
        )
    }

    fn decode_get_accounts_response(&self, response_data: &[u8]) -> Result<GetAccountsRet, String> {
        Decode!(response_data, GetAccountsRet).map_err(|error| format!("{error:?}"))
    }

    fn get_delegation_signature_msg(
        &self,
        public_key: &[u8],
        expiration: u64,
        targets: Option<&Vec<Vec<u8>>>,
    ) -> Vec<u8> {
        let mut m: Vec<(String, Value)> = vec![];
        m.push(("pubkey".into(), Value::Bytes(public_key.to_vec())));
        m.push(("expiration".into(), Value::Number(expiration)));
        if let Some(targets) = targets.as_ref() {
            let mut arr = Vec::with_capacity(targets.len());
            for t in targets.iter() {
                arr.push(Value::Bytes(t.to_vec()));
            }
            m.push(("targets".into(), Value::Array(arr)));
        }
        let hash = representation_independent_hash(m.as_slice()).to_vec();

        let mut msg = vec![];
        msg.extend([DELEGATION_SIG_DOMAIN.len() as u8]);
        msg.extend(DELEGATION_SIG_DOMAIN.to_vec());
        msg.extend(hash);
        msg
    }
}
