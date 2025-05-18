use crate::log;
use crate::simulations::discovery::Discovery;
use crate::utils::args::Args;
use crate::utils::logger::init_logger;
use crate::utils::io::directory_manager::DirectoryManager;

pub struct App {
    args: Args,
    discovery: Discovery,
}

impl App {
    pub fn new(args: Args) -> Self {
        let results_dir = args.results_dir.clone();
        let directory_manager = DirectoryManager::new(&results_dir);

        Self::assure_results_directory(&directory_manager);

        let global_path = directory_manager.get_global_path();

        App {
            args,
            discovery: Discovery::new(&global_path),
        }
    }

    pub fn run(&self) {
        init_logger(self.args.verbose);

        match self.args.technique.to_lowercase().as_str() {
            "t1087" | "discovery" => self.discovery.run(),

            _ => log!("Unknown technique: {}", self.args.technique),
        }
    }

    fn assure_results_directory(directory_manager: &DirectoryManager)
    {
        let results_dir = directory_manager.get_global_path();
        if directory_manager.directory_exists() {
            log!("Results directory already exists: {}", results_dir);
            return;
        }

        match directory_manager.create_directory() {
            Ok(_) => log!("Created results directory: {}", results_dir),
            Err(e) => log!("Failed to create results directory: {}. Error: {}", results_dir, e),
        }
    }
}
