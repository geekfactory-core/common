use crate::components::{rand::RandGenerator, time::Time};
use std::time::Duration;

pub async fn sleep(rand_generator: &dyn RandGenerator, time: &dyn Time, duration: Duration) {
    let start = time.get_current_unix_epoch_time_nanos();
    let mut sleep_count = 1;
    loop {
        sleep_count += 1;

        let _ = rand_generator.generate_32().await;

        let sleep_total = time.get_current_unix_epoch_time_nanos() - start;
        if sleep_count > 100 || duration.as_nanos() < sleep_total {
            return;
        }
    }
}

// pub async fn wait_response_with_timer(duration: Duration) {
//     // creates synchronous channel
//     let (sender, receiver) = futures::channel::oneshot::channel();
//     let rc_sender = Arc::new(Mutex::new(Some(sender)));
//     let weak_sender = Arc::downgrade(&rc_sender);

//     // registers send after duration on ic timer
//     ic_cdk_timers::set_timer(duration, move || {
//         if let Some(rc) = weak_sender.upgrade() {
//             if let Ok(mut maybe_sender) = rc.try_lock() {
//                 if let Some(sender) = maybe_sender.take() {
//                     let _ = sender.send(());
//                 }
//             }
//         }
//     });

//     // suspends until message is received
//     receiver.await.ok();
// }
