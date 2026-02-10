use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub struct ChunkDef {
    pub start: usize,
    pub count: usize,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub enum SortingOrder {
    Ascending,
    Descending,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SortingDefinition<K> {
    pub key: K,
    pub order: SortingOrder,
}
