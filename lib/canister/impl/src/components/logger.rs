use ic_cdk::api::debug_print;

pub trait Logger {
    fn debug(&self, message: &str);
    fn info(&self, message: &str);
    fn error(&self, error: &str);
}

pub struct LocalLoggerImpl;

impl Logger for LocalLoggerImpl {
    fn debug(&self, _message: &str) {
        #[cfg(any(network = "local", network = "test"))]
        debug_print(format!("DEBUG {_message}"));
    }

    fn info(&self, message: &str) {
        debug_print(format!("INFO {message}"));
    }

    fn error(&self, error: &str) {
        debug_print(format!("ERROR {error}"));
    }
}
