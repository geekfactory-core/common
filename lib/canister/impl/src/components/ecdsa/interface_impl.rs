use async_trait::async_trait;
use common_canister_types::{DerivationPath, EcdsaKeyCompact, EcdsaSignatureCompact, MessageHash};
use ic_cdk::management_canister::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgs,
    SignWithEcdsaArgs,
};

use super::interface::EcdsaSignature;

pub struct EcdsaSignatureImpl {
    ecdsa_key_id: EcdsaKeyId,
}

impl EcdsaSignatureImpl {
    pub fn new(key_name: String) -> Self {
        Self {
            ecdsa_key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name,
            },
        }
    }
}

#[async_trait]
impl EcdsaSignature for EcdsaSignatureImpl {
    fn get_ecdsa_key_id(&self) -> EcdsaKeyId {
        self.ecdsa_key_id.clone()
    }

    async fn get_ecdsa_key(
        &self,
        derivation_path: DerivationPath,
    ) -> Result<EcdsaKeyCompact, String> {
        let arg = EcdsaPublicKeyArgs {
            canister_id: None,
            derivation_path,
            key_id: self.ecdsa_key_id.clone(),
        };

        ecdsa_public_key(&arg)
            .await
            .map(|result| result.public_key)
            .map_err(|error| format!("Error while getting ECDSA key, reason: {error:?}"))
    }

    async fn sign_with_ecdsa(
        &self,
        derivation_path: DerivationPath,
        message_hash: &MessageHash,
    ) -> Result<EcdsaSignatureCompact, String> {
        let arg = SignWithEcdsaArgs {
            message_hash: message_hash.to_vec(),
            derivation_path,
            key_id: self.ecdsa_key_id.clone(),
        };

        sign_with_ecdsa(&arg)
            .await
            .map(|result| result.signature)
            .map_err(|error| format!("Error while signing with ECDSA, reason: {error:?}"))
    }
}
