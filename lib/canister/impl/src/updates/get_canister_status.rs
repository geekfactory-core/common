#[macro_export]
macro_rules! get_canister_status {
    () => {
        use common_canister_api::get_canister_status::*;
        use ic_cdk::management_canister::canister_status;
        use ic_cdk::{api::canister_self, management_canister::CanisterIdRecord};

        #[ic_cdk_macros::update]
        async fn get_canister_status() -> Response {
            let canister_id = canister_self();
            canister_status(&CanisterIdRecord { canister_id })
                .await
                .map(|result| GetCanisterStatusResult {
                    canister_status_response: result,
                })
                .map_err(|error| GetCanisterStatusError::ManagementCallError {
                    reason: format!("{:?}", error),
                })
                .into()
        }
    };
}
