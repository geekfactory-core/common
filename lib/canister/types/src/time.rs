use candid::{CandidType, Deserialize};
use serde::Serialize;

pub type TimestampSeconds = u64;
pub type TimestampMillis = u64;
pub type TimestampNanos = u128;
pub type TimestampNanosClipped = u64;

const NANOS_PER_MILLISECOND: u128 = 1_000_000;

pub fn nanos_to_millis(nanos: &TimestampNanos) -> TimestampMillis {
    (nanos / NANOS_PER_MILLISECOND) as u64
}

pub fn millis_to_nanos(millis: &TimestampMillis) -> TimestampNanos {
    (*millis as u128) * NANOS_PER_MILLISECOND
}

#[derive(CandidType, Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct DelayedTimestampMillis {
    pub time: TimestampMillis,
    pub delay: TimestampMillis,
}
