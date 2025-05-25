use crate::utils::args::Args;
use crate::utils::io::directory_manager::DirectoryManager;
use crate::simulations::{
    discovery::Discovery,
    persistence::Persistence,
    execution::Execution,
    ssh::Ssh
};
use crate::utils::logger::{
    Logger,
    set_verbose
};

pub struct App {
    args: Args,
    logger: Logger,
    discovery: Discovery,
    persistence: Persistence,
    execution: Execution,
    ssh: Ssh,
}

impl App {
    pub fn new(args: Args) -> Self {
        set_verbose(args.verbose);

        let logger = Logger::new("App");
        let global_path = Self::get_results_directory_global_path(&args.results_dir, &logger);

        App {
            args,
            logger,
            discovery: Discovery::new(&global_path),
            persistence: Persistence::new(&global_path),
            execution: Execution::new(&global_path),
            ssh: Ssh::new(&global_path),
        }
    }

    pub fn run(&self) {
        self.logger.info("Starting application...");

        match self.args.technique.to_lowercase().as_str() {
            "t1087" | "discovery" => self.discovery.run(),
            "t1543.003" | "persistence" => self.persistence.run(),
            "t1059.004" | "execution" => self.execution.run(),
            "t1021.004" | "ssh" => self.ssh.run(),

            _ => self.logger.error(format!("Unknown technique: {}", self.args.technique.to_string())),
        }
    }

    fn get_results_directory_global_path(results_dir: &str, logger: &Logger) -> String
    {
        let directory_manager = DirectoryManager::new(&results_dir.to_string());
        let results_dir = directory_manager.get_global_path();

        if !directory_manager.directory_exists() {
            match directory_manager.create_directory() {
                Ok(_) => logger.info(format!("Created results directory: {}", results_dir)),
                Err(e) => logger.error(format!("Failed to create results directory: {}. Error: {}", results_dir, e)),
            }
        }

        directory_manager.get_global_path()
    }
}
