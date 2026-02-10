use async_trait::async_trait;
use ic_cdk::{
    call::CallResult,
    management_canister::{
        canister_status, clear_chunk_store, create_canister_with_extra_cycles,
        install_chunked_code, stored_chunks, update_settings, upload_chunk, CanisterIdRecord,
        CanisterStatusArgs, CanisterStatusResult, ChunkHash, ClearChunkStoreArgs,
        CreateCanisterArgs, InstallChunkedCodeArgs, StoredChunksArgs, UpdateSettingsArgs,
        UploadChunkArgs,
    },
};

#[async_trait]
pub trait IcManagement {
    async fn canister_status(&self, arg: CanisterStatusArgs) -> CallResult<CanisterStatusResult>;

    async fn clear_chunk_store(&self, arg: ClearChunkStoreArgs) -> CallResult<()>;

    async fn upload_chunk(&self, arg: UploadChunkArgs) -> CallResult<ChunkHash>;

    async fn stored_chunks(&self, arg: StoredChunksArgs) -> CallResult<Vec<ChunkHash>>;

    async fn install_chunked_code(&self, arg: InstallChunkedCodeArgs) -> CallResult<()>;

    async fn create_canister_with_extra_cycles(
        &self,
        arg: CreateCanisterArgs,
        extra_cycles: u128,
    ) -> CallResult<CanisterIdRecord>;

    async fn update_settings(&self, arg: UpdateSettingsArgs) -> CallResult<()>;
}

pub struct IcManagementImpl {}

#[async_trait]
impl IcManagement for IcManagementImpl {
    async fn canister_status(&self, arg: CanisterStatusArgs) -> CallResult<CanisterStatusResult> {
        canister_status(&arg).await
    }

    async fn clear_chunk_store(&self, arg: ClearChunkStoreArgs) -> CallResult<()> {
        clear_chunk_store(&arg).await
    }

    async fn upload_chunk(&self, arg: UploadChunkArgs) -> CallResult<ChunkHash> {
        upload_chunk(&arg).await
    }

    async fn stored_chunks(&self, arg: StoredChunksArgs) -> CallResult<Vec<ChunkHash>> {
        stored_chunks(&arg).await
    }

    async fn install_chunked_code(&self, arg: InstallChunkedCodeArgs) -> CallResult<()> {
        install_chunked_code(&arg).await
    }

    async fn create_canister_with_extra_cycles(
        &self,
        arg: CreateCanisterArgs,
        extra_cycles: u128,
    ) -> CallResult<CanisterIdRecord> {
        create_canister_with_extra_cycles(&arg, extra_cycles).await
    }

    async fn update_settings(&self, arg: UpdateSettingsArgs) -> CallResult<()> {
        update_settings(&arg).await
    }
}
