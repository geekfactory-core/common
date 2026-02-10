use common_canister_types::{
    nanos_to_millis, DelayedTimestampMillis, TimestampMillis, TimestampNanos,
};

pub trait Time: Sync + Send {
    fn get_current_unix_epoch_time_nanos(&self) -> TimestampNanos;

    fn get_current_unix_epoch_time_millis(&self) -> TimestampMillis;

    fn get_delayed_time_millis(&self, time: TimestampMillis) -> DelayedTimestampMillis;

    fn get_delayed_time_by_delay_millis(&self, delay: TimestampMillis) -> DelayedTimestampMillis;
}

pub struct TimeImpl;

impl Time for TimeImpl {
    fn get_current_unix_epoch_time_nanos(&self) -> TimestampNanos {
        #[cfg(target_arch = "wasm32")]
        {
            ic_cdk::api::time().into()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time wrapped around.")
                .as_nanos()
        }
    }

    fn get_current_unix_epoch_time_millis(&self) -> TimestampMillis {
        nanos_to_millis(&self.get_current_unix_epoch_time_nanos())
    }

    fn get_delayed_time_millis(&self, time: TimestampMillis) -> DelayedTimestampMillis {
        let now = self.get_current_unix_epoch_time_millis();
        DelayedTimestampMillis {
            time,
            delay: time.saturating_sub(now),
        }
    }

    fn get_delayed_time_by_delay_millis(&self, delay: TimestampMillis) -> DelayedTimestampMillis {
        let now = self.get_current_unix_epoch_time_millis();
        DelayedTimestampMillis {
            time: now + delay,
            delay,
        }
    }
}
