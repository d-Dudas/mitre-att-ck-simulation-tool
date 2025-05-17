use crate::args::Args;
use crate::log;
use crate::simulations::discovery::Discovery;
use crate::utils::logger::init_logger;

pub struct App {
    discovery: Discovery,
}

impl App {
    pub fn new() -> Self {
        App {
            discovery: Discovery::new(),
        }
    }

    pub fn run(&self, args: Args) {
        init_logger(args.verbose);
        match args.technique.to_lowercase().as_str() {
            "t1087" => self.discovery.run(),
            "discovery" => self.discovery.run(),
            _ => log!("Unknown technique: {}", args.technique),
        }
    }
}
