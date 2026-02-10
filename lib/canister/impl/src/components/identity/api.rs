use candid::{self, CandidType, Deserialize, Principal};
use common_canister_types::components::identity::{Iss, Sub, Timestamp};
use ic_cdk::call::CallResult;

pub const INTERNET_IDENTITY_CANISTER_ID: &str = "rdmx6-jaaaa-aaaaa-aaadq-cai";

#[derive(CandidType, Deserialize, Debug)]
pub enum MetadataMapV2Item1 {
    Map(Box<MetadataMapV2>),
    String(String),
    Bytes(serde_bytes::ByteBuf),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct MetadataMapV2(pub Vec<(String, MetadataMapV2Item1)>);
pub type Aud = String;

#[derive(CandidType, Deserialize, Debug)]
pub struct OpenIdCredential {
    pub aud: Aud,
    pub iss: Iss,
    pub sub: Sub,
    pub metadata: Box<MetadataMapV2>,
    pub last_usage_timestamp: Option<Timestamp>,
}

pub type UserNumber = u64;
#[derive(CandidType, Deserialize, Debug)]
pub enum MetadataMapItem1 {
    #[serde(rename = "map")]
    Map(Box<MetadataMap>),
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "bytes")]
    Bytes(serde_bytes::ByteBuf),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct MetadataMap(Vec<(String, MetadataMapItem1)>);

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub enum DeviceProtection {
    #[serde(rename = "unprotected")]
    Unprotected,
    #[serde(rename = "protected")]
    Protected,
}

pub type PublicKey = serde_bytes::ByteBuf;
pub type DeviceKey = PublicKey;
#[derive(CandidType, Deserialize, Debug)]
pub enum KeyType {
    #[serde(rename = "platform")]
    Platform,
    #[serde(rename = "seed_phrase")]
    SeedPhrase,
    #[serde(rename = "cross_platform")]
    CrossPlatform,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "browser_storage_key")]
    BrowserStorageKey,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Purpose {
    #[serde(rename = "authentication")]
    Authentication,
    #[serde(rename = "recovery")]
    Recovery,
}

pub type CredentialId = serde_bytes::ByteBuf;
#[derive(CandidType, Deserialize, Debug)]
pub struct DeviceData {
    pub alias: String,
    pub metadata: Option<Box<MetadataMap>>,
    pub origin: Option<String>,
    pub protection: DeviceProtection,
    pub pubkey: DeviceKey,
    pub key_type: KeyType,
    pub purpose: Purpose,
    pub credential_id: Option<CredentialId>,
}

#[derive(CandidType, Deserialize)]
pub enum AddTentativeDeviceResponse {
    #[serde(rename = "device_registration_mode_off")]
    DeviceRegistrationModeOff,
    #[serde(rename = "another_device_tentatively_added")]
    AnotherDeviceTentativelyAdded,
    #[serde(rename = "added_tentatively")]
    AddedTentatively {
        verification_code: String,
        device_registration_timeout: Timestamp,
    },
}

pub type IdentityNumber = u64;
#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub enum AuthnMethodProtection {
    Protected,
    Unprotected,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum AuthnMethodPurpose {
    Recovery,
    Authentication,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct AuthnMethodSecuritySettings {
    pub protection: AuthnMethodProtection,
    pub purpose: AuthnMethodPurpose,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct PublicKeyAuthn {
    pub pubkey: PublicKey,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct WebAuthn {
    pub pubkey: PublicKey,
    pub credential_id: CredentialId,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum AuthnMethod {
    PubKey(PublicKeyAuthn),
    WebAuthn(WebAuthn),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct AuthnMethodData {
    pub security_settings: AuthnMethodSecuritySettings,
    pub metadata: Box<MetadataMapV2>,
    pub last_authentication: Option<Timestamp>,
    pub authn_method: AuthnMethod,
}

#[derive(CandidType, Deserialize)]
pub enum AuthnMethodConfirmationError {
    Unauthorized(Principal),
    RegistrationModeOff,
    NoAuthnMethodToConfirm,
    WrongCode { retries_left: u8 },
    InternalCanisterError(String),
}

#[derive(CandidType, Deserialize)]
pub enum AuthnMethodConfirmRet {
    Ok,
    Err(AuthnMethodConfirmationError),
}

#[derive(CandidType, Deserialize)]
pub struct AuthnMethodConfirmationCode {
    pub confirmation_code: String,
    pub expiration: Timestamp,
}

#[derive(CandidType, Deserialize)]
pub enum AuthnMethodRegisterError {
    RegistrationModeOff,
    RegistrationAlreadyInProgress,
    InvalidMetadata(String),
}

#[derive(CandidType, Deserialize)]
pub enum AuthnMethodRegisterRet {
    Ok(AuthnMethodConfirmationCode),
    Err(AuthnMethodRegisterError),
}

#[derive(CandidType, Deserialize)]
pub enum AuthnMethodRegistrationModeEnterRet {
    Ok { expiration: Timestamp },
    Err(AuthnMethodRegistrationModeEnterError),
}

pub type RegistrationId = String;
#[derive(CandidType, Deserialize, PartialEq, Debug)]
pub enum AuthnMethodRegistrationModeEnterError {
    InvalidRegistrationId(String),
    InternalCanisterError(String),
    AlreadyInProgress,
    Unauthorized(Principal),
}

pub type AuthnMethodRegistrationModeExitRet =
    std::result::Result<(), AuthnMethodRegistrationModeExitError>;

#[derive(CandidType, Deserialize, PartialEq, Debug)]
pub enum AuthnMethodRegistrationModeExitError {
    Unauthorized(Principal),
    InternalCanisterError(String),
    RegistrationModeOff,
    InvalidMetadata(String),
}

#[derive(CandidType, Deserialize)]
pub enum AuthnMethodRemoveRet {
    Ok,
    Err,
}

#[derive(CandidType, Deserialize)]
pub struct Delegation {
    pub pubkey: PublicKey,
    pub targets: Option<Vec<Principal>>,
    pub expiration: Timestamp,
}

#[derive(CandidType, Deserialize)]
pub struct SignedDelegation {
    pub signature: serde_bytes::ByteBuf,
    pub delegation: Delegation,
}

#[derive(CandidType, Deserialize)]
pub enum GetDelegationResponse {
    #[serde(rename = "no_such_delegation")]
    NoSuchDelegation,
    #[serde(rename = "signed_delegation")]
    SignedDelegation(SignedDelegation),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct IdentityAuthnInfo {
    pub authn_methods: Vec<AuthnMethod>,
    pub recovery_authn_methods: Vec<AuthnMethod>,
}

#[derive(CandidType, Deserialize)]
pub enum IdentityAuthnInfoRet {
    Ok(IdentityAuthnInfo),
    Err,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct AuthnMethodRegistrationInfo {
    pub expiration: Timestamp,
    pub authn_method: Option<AuthnMethodData>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct IdentityInfo {
    pub authn_methods: Vec<AuthnMethodData>,
    pub metadata: Box<MetadataMapV2>,
    pub authn_method_registration: Option<AuthnMethodRegistrationInfo>,
    pub openid_credentials: Option<Vec<OpenIdCredential>>,
    // new fields
    pub name: Option<String>,
    pub created_at: Option<Timestamp>,
}

#[derive(CandidType, Deserialize)]
pub enum IdentityInfoError {
    InternalCanisterError(String),
    Unauthorized(Principal),
}

#[derive(CandidType, Deserialize)]
pub enum IdentityInfoRet {
    Ok(IdentityInfo),
    Err(IdentityInfoError),
}

#[derive(CandidType, Deserialize)]
pub enum OpenIdCredentialRemoveError {
    InternalCanisterError(String),
    OpenIdCredentialNotFound,
    Unauthorized(Principal),
}

#[derive(CandidType, Deserialize)]
pub enum OpenidCredentialRemoveRet {
    Ok,
    Err(OpenIdCredentialRemoveError),
}

pub struct Service(pub Principal);
impl Service {
    pub async fn authn_method_register(
        &self,
        arg0: IdentityNumber,
        arg1: AuthnMethodData,
    ) -> CallResult<AuthnMethodRegisterRet> {
        Ok(
            ic_cdk::call::Call::bounded_wait(self.0, "authn_method_register")
                .with_args(&(arg0, arg1))
                .await?
                .candid()?,
        )
    }

    pub async fn identity_authn_info(
        &self,
        arg0: IdentityNumber,
    ) -> CallResult<IdentityAuthnInfoRet> {
        Ok(
            ic_cdk::call::Call::bounded_wait(self.0, "identity_authn_info")
                .with_arg(arg0)
                .await?
                .candid()?,
        )
    }
}
