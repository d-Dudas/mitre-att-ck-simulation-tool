use std::sync::atomic::{AtomicBool, Ordering};

pub static VERBOSE: AtomicBool = AtomicBool::new(false);


pub struct Logger {
    prefix: String,
}

impl Logger {

    pub fn new(prefix: &str) -> Self {
        Logger {
            prefix: prefix.to_string(),
        }
    }

    fn log(&self, format: &str, args: impl std::fmt::Display) {
        println!("{} [{}]: {}", Self::get_timestamp(), self.prefix, format.replace("{}", &args.to_string()));
    }

    pub fn info(&self, args: impl std::fmt::Display) {
        self.log("{}", args);
    }

    pub fn error(&self, args: impl std::fmt::Display) {
        eprintln!("{} [{}]: {}", Self::get_timestamp(), self.prefix, args);
    }

    pub fn verbose(&self, args: impl std::fmt::Display) {
        if VERBOSE.load(Ordering::SeqCst) {
            self.log("{}", args);
        }
    }

    fn get_timestamp() -> String {
        let now = chrono::Local::now();
        now.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

pub fn set_verbose(verbose: bool) {
    VERBOSE.store(verbose, Ordering::SeqCst);
}
