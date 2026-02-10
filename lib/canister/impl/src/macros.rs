#[macro_export]
macro_rules! canister_state {
    ($type:ty) => {
        use std::cell::RefCell;

        thread_local! {
            static __STATE: RefCell<Option<$type>> = RefCell::default();
        }

        const __STATE_ALREADY_INITIALIZED: &str = "State is already initialized";
        const __STATE_NOT_INITIALIZED: &str = "State is not initialized";

        fn init_state(state: $type) {
            __STATE.with(|s| {
                if s.borrow().is_some() {
                    panic!("{}", __STATE_ALREADY_INITIALIZED);
                } else {
                    *s.borrow_mut() = Some(state);
                }
            });
        }

        fn take_state() -> $type {
            __STATE.with(|s| s.take()).expect(__STATE_NOT_INITIALIZED)
        }

        fn read_state<F, R>(f: F) -> R
        where
            F: FnOnce(&$type) -> R,
        {
            __STATE.with(|s| f(s.borrow().as_ref().expect(__STATE_NOT_INITIALIZED)))
        }

        fn mutate_state<F, R>(f: F) -> R
        where
            F: FnOnce(&mut $type) -> R,
        {
            __STATE.with(|s| f(s.borrow_mut().as_mut().expect(__STATE_NOT_INITIALIZED)))
        }
    };
}
