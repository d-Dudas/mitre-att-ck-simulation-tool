use std::sync::atomic::{
    AtomicBool,
    Ordering
};

pub static VERBOSE: AtomicBool = AtomicBool::new(false);

const INFO_LEVEL_PREFIX: &str = "INF";
const ERROR_LEVEL_PREFIX: &str = "ERR";
const VERBOSE_LEVEL_PREFIX: &str = "VER";

pub struct Logger {
    prefix: String,
}

impl Logger {

    pub fn new(prefix: &str) -> Self {
        Logger {
            prefix: prefix.to_string(),
        }
    }

    fn log(&self, format: &str, args: impl std::fmt::Display, level_prefix: &str) {
        println!("{} [{}][{}]: {}", Self::get_timestamp(), level_prefix, self.prefix, format.replace("{}", &args.to_string()));
    }

    pub fn info(&self, args: impl std::fmt::Display) {
        self.log("{}", args, INFO_LEVEL_PREFIX);
    }

    pub fn error(&self, args: impl std::fmt::Display) {
        eprintln!("{} [{}][{}]: {}", Self::get_timestamp(), ERROR_LEVEL_PREFIX, self.prefix, args);
    }

    pub fn verbose(&self, args: impl std::fmt::Display) {
        if VERBOSE.load(Ordering::SeqCst) {
            self.log("{}", args, VERBOSE_LEVEL_PREFIX);
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
