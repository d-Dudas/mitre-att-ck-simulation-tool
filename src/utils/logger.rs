use lazy_static::lazy_static;
use parking_lot::Mutex;

pub struct Logger {
    verbose: bool,
}

impl Logger {
    pub fn new(verbose: bool) -> Self {
        Logger { verbose }
    }

    pub fn log(&self, message: &str) {
        println!("{}", message);
    }

    pub fn log_error(&self, message: &str) {
        eprintln!("Error: {}", message);
    }

    pub fn log_verbose(&self, message: &str) {
        if self.verbose {
            println!("Verbose: {}", message);
        }
    }
}

lazy_static! {
    pub static ref LOGGER: Mutex<Option<Logger>> = Mutex::new(None);
}

pub fn init_logger(verbose: bool) {
    let mut logger = LOGGER.lock();
    *logger = Some(Logger::new(verbose));
}

pub fn get_logger() -> Option<parking_lot::MutexGuard<'static, Option<Logger>>> {
    let logger = LOGGER.lock();
    if logger.is_some() { Some(logger) } else { None }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ({
        let logger = $crate::utils::logger::get_logger();
        if let Some(guard) = logger {
            if let Some(ref logger) = *guard {
                logger.log(format!($($arg)*).as_str());
            }
        }
    })
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => ({
        let logger = $crate::utils::logger::get_logger();
        if let Some(guard) = logger {
            if let Some(ref logger) = *guard {
                logger.log_error(format!("Error: {}", format!($($arg)*)).as_str());
            }
        }
    })
}

#[macro_export]
macro_rules! log_verbose {
    ($($arg:tt)*) => ({
        let logger = $crate::utils::logger::get_logger();
        if let Some(guard) = logger {
            if let Some(ref logger) = *guard {
                logger.log_verbose(format!("Verbose: {}", format!($($arg)*)).as_str());
            }
        }
    })
}
