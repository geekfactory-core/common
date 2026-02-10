use candid::Principal;
use common_canister_types::CanisterMetrics;
use ic_cdk::{
    api::{
        canister_cycle_balance, canister_self, certified_data_set, cost_create_canister, msg_caller,
    },
    stable::{stable_size, WASM_PAGE_SIZE_IN_BYTES},
};

pub trait Ic {
    fn get_root_public_key_raw(&self) -> &[u8];
    fn get_canister(&self) -> Principal;
    fn get_canister_metrics(&self) -> CanisterMetrics;

    fn get_caller(&self) -> Principal;
    fn is_caller_anonymous(&self) -> bool;

    fn set_certified_data(&self, data: &[u8]);
    fn get_cost_create_canister(&self) -> u128;
}

pub struct IcImpl {
    root_public_key_raw: Vec<u8>,
}

impl IcImpl {
    pub fn new(root_public_key_raw: Vec<u8>) -> Self {
        Self {
            root_public_key_raw,
        }
    }
}

impl Ic for IcImpl {
    fn get_root_public_key_raw(&self) -> &[u8] {
        &self.root_public_key_raw
    }

    fn get_canister(&self) -> Principal {
        canister_self()
    }

    fn get_canister_metrics(&self) -> CanisterMetrics {
        CanisterMetrics {
            stable_memory_size: stable_size() * WASM_PAGE_SIZE_IN_BYTES,
            heap_memory_size: get_heap_memory_size(),
            cycles: canister_cycle_balance(),
        }
    }

    fn get_caller(&self) -> Principal {
        msg_caller()
    }

    fn is_caller_anonymous(&self) -> bool {
        is_principal_anonymous(&self.get_caller())
    }

    fn set_certified_data(&self, data: &[u8]) {
        certified_data_set(data);
    }

    fn get_cost_create_canister(&self) -> u128 {
        cost_create_canister()
    }
}

pub fn is_principal_anonymous(principal: &Principal) -> bool {
    *principal.as_slice() == vec![4]
}

#[cfg(test)]
mod tests {
    use super::is_principal_anonymous;
    use candid::Principal;

    #[test]
    fn test() {
        assert!(is_principal_anonymous(&Principal::anonymous()));
        assert!(!is_principal_anonymous(&Principal::management_canister()));
    }
}

fn get_heap_memory_size() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        (core::arch::wasm32::memory_size(0) as u64) * ic_cdk::stable::WASM_PAGE_SIZE_IN_BYTES
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}
