use async_trait::async_trait;
use common_canister_types::{DerivationPath, EcdsaKeyCompact, EcdsaSignatureCompact, MessageHash};
use ic_cdk::management_canister::EcdsaKeyId;

#[async_trait]
pub trait EcdsaSignature: Sync + Send {
    fn get_ecdsa_key_id(&self) -> EcdsaKeyId;

    async fn get_ecdsa_key(
        &self,
        derivation_path: DerivationPath,
    ) -> Result<EcdsaKeyCompact, String>;

    async fn sign_with_ecdsa(
        &self,
        derivation_path: DerivationPath,
        message_hash: &MessageHash,
    ) -> Result<EcdsaSignatureCompact, String>;
}
