use std::time::Duration;

use ic_cdk_timers::TimerId;

pub trait Timer: Sync + Send {
    fn clear_timer(&self, timer_id: TimerId);

    fn set_timer(&self, delay: Duration, func: Box<dyn Fn()>) -> Option<TimerId>;
}

pub struct TimerImpl;

impl Timer for TimerImpl {
    fn clear_timer(&self, timer_id: TimerId) {
        ic_cdk_timers::clear_timer(timer_id);
    }

    fn set_timer(&self, delay: Duration, func: Box<dyn Fn()>) -> Option<TimerId> {
        let future = async move {
            func();
        };
        Some(ic_cdk_timers::set_timer(delay, future))
    }
}
