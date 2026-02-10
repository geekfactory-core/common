use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct CanisterMetrics {
    pub stable_memory_size: u64,
    pub heap_memory_size: u64,
    pub cycles: u128,
}
