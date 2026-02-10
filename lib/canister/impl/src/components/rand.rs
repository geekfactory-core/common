use async_trait::async_trait;
use ic_cdk::management_canister::raw_rand;

#[async_trait]
pub trait RandGenerator: Sync + Send {
    async fn generate_16(&self) -> Result<Vec<u8>, String>;

    async fn generate_32(&self) -> Result<Vec<u8>, String>;
}

pub struct IcRandGenerator;

#[async_trait]
impl RandGenerator for IcRandGenerator {
    async fn generate_16(&self) -> Result<Vec<u8>, String> {
        self.generate_32()
            .await
            .map(|v| v.as_slice()[..16].to_vec())
    }

    async fn generate_32(&self) -> Result<Vec<u8>, String> {
        raw_rand()
            .await
            .map_err(|error| format!("Error while generating random bytes, reason: {error:?}"))
    }
}
