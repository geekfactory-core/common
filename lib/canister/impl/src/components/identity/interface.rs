use std::time::Duration;

use async_trait::async_trait;
use candid::Principal;
use common_canister_types::{components::identity::OpenIdCredentialKey, TimestampNanos};

use crate::{
    components::identity::api::RegistrationId,
    handlers::{ic_request::builder::CanisterRequest, IcAgentRequestDefinition},
};

use super::api::{
    AccountNumber, AuthnMethodConfirmRet, AuthnMethodData, AuthnMethodRegisterRet,
    AuthnMethodRegistrationModeEnterRet, AuthnMethodRegistrationModeExitRet, AuthnMethodRemoveRet,
    GetAccountsRet, GetDefaultAccountRet, GetDelegationResponse, IdentityAuthnInfoRet,
    IdentityInfoRet, IdentityNumber, OpenidCredentialRemoveRet, PrepareAccountDelegationRet,
    PublicKey, UserNumber,
};

#[async_trait]
pub trait Identity: Sync + Send {
    async fn identity_authn_info(
        &self,
        identity_number: IdentityNumber,
    ) -> Result<IdentityAuthnInfoRet, String>;

    fn build_identity_info_request(
        &self,
        identity_number: &IdentityNumber,
    ) -> IcAgentRequestDefinition;

    fn decode_identity_info_response(
        &self,
        response_data: &[u8],
    ) -> Result<IdentityInfoRet, String>;

    async fn authn_method_register(
        &self,
        identity_number: IdentityNumber,
        authn_method_data: AuthnMethodData,
    ) -> Result<AuthnMethodRegisterRet, String>;

    fn build_authn_method_session_register(
        &self,
        identity_number: &IdentityNumber,
    ) -> IcAgentRequestDefinition;

    fn decode_authn_method_session_register(
        &self,
        response_data: &[u8],
    ) -> Result<AuthnMethodRegisterRet, String>;

    fn build_authn_method_remove_request(
        &self,
        identity_number: &IdentityNumber,
        public_key: &PublicKey,
    ) -> IcAgentRequestDefinition;

    fn decode_authn_method_remove_response(
        &self,
        response_data: &[u8],
    ) -> Result<AuthnMethodRemoveRet, String>;

    fn build_authn_method_registration_mode_exit_request(
        &self,
        identity_number: &IdentityNumber,
        auth_method_data: &Option<AuthnMethodData>,
    ) -> IcAgentRequestDefinition;

    fn decode_authn_method_registration_mode_exit_response(
        &self,
        response_data: &[u8],
    ) -> Result<AuthnMethodRegistrationModeExitRet, String>;

    fn build_openid_credential_remove_request(
        &self,
        identity_number: &IdentityNumber,
        key: &OpenIdCredentialKey,
    ) -> IcAgentRequestDefinition;

    fn decode_openid_credential_remove_response(
        &self,
        response_data: &[u8],
    ) -> Result<OpenidCredentialRemoveRet, String>;

    #[allow(clippy::ptr_arg)]
    fn build_get_principal_request(
        &self,
        user_number: &UserNumber,
        frontend_hostname: &String,
    ) -> IcAgentRequestDefinition;

    fn decode_get_principal_response(&self, response_data: &[u8]) -> Result<Principal, String>;

    fn build_authn_method_registration_mode_enter_request(
        &self,
        identity_number: &IdentityNumber,
        registration_id: &Option<RegistrationId>,
    ) -> IcAgentRequestDefinition;

    fn decode_authn_method_registration_mode_enter_response(
        &self,
        response_data: &[u8],
    ) -> Result<AuthnMethodRegistrationModeEnterRet, String>;

    fn build_authn_method_confirm_request(
        &self,
        identity_number: &IdentityNumber,
        verification_code: String,
    ) -> IcAgentRequestDefinition;

    fn decode_authn_method_confirm_response(
        &self,
        response_data: &[u8],
    ) -> Result<AuthnMethodConfirmRet, String>;

    fn build_prepare_delegation_request(
        &self,
        user_number: &UserNumber,
        frontend_hostname: String,
        session_key: Vec<u8>,
        delegation_duration: Duration,
    ) -> IcAgentRequestDefinition;

    fn decode_prepare_delegation_response(
        &self,
        response_data: &[u8],
    ) -> Result<(Vec<u8>, TimestampNanos), String>;

    fn build_prepare_account_delegation_request(
        &self,
        user_number: &UserNumber,
        frontend_hostname: String,
        account_number: Option<AccountNumber>,
        session_key: Vec<u8>,
        delegation_duration: Duration,
    ) -> IcAgentRequestDefinition;

    fn decode_prepare_account_delegation_response(
        &self,
        response_data: &[u8],
    ) -> Result<PrepareAccountDelegationRet, String>;

    fn build_get_delegation_request(
        &self,
        user_number: &UserNumber,
        frontend_hostname: String,
        session_key: Vec<u8>,
        timestamp: TimestampNanos,
    ) -> CanisterRequest;

    fn decode_get_delegation_response(
        &self,
        response_data: &[u8],
    ) -> Result<GetDelegationResponse, String>;

    fn build_get_default_account_request(
        &self,
        user_number: &UserNumber,
        frontend_hostname: String,
    ) -> IcAgentRequestDefinition;

    fn decode_get_default_account_response(
        &self,
        response_data: &[u8],
    ) -> Result<GetDefaultAccountRet, String>;

    fn build_get_accounts_request(
        &self,
        user_number: &UserNumber,
        frontend_hostname: String,
    ) -> IcAgentRequestDefinition;

    fn decode_get_accounts_response(&self, response_data: &[u8]) -> Result<GetAccountsRet, String>;

    fn get_delegation_signature_msg(
        &self,
        public_key: &[u8],
        expiration: u64,
        targets: Option<&Vec<Vec<u8>>>,
    ) -> Vec<u8>;
}
