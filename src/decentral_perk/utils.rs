use crate::types::Epoch;

const NANOS_PER_SEC: u64 = 1_000_000_000;

/// Gets the current time in seconds.
pub fn time_secs() -> Epoch {
    let now: u64 = ic_cdk::api::time();
    now / NANOS_PER_SEC 
}