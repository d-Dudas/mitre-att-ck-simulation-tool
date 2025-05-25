use std::process::Command;

use crate::utils::{
    io::file_writer::FileWriter,
    logger::Logger
};

const EXECUTION_LOG_FILE: &str = "execution.log";
const TEMPORARY_OUTPUT_FILE: &str = "/tmp/cmd_sim_output.txt";

pub struct Execution {
    logger: Logger,
    file_writer: FileWriter,
}

impl Execution {
    pub fn new(results_directory_global_path: &str) -> Self {
        let result_path = format!("{}/{}", results_directory_global_path, EXECUTION_LOG_FILE);
        Execution {
            file_writer: FileWriter::new(result_path),
            logger: Logger::new("Execution"),
        }
    }

    pub fn run(&self) {
        self.logger.info("Simulating Execution...");

        self.execute_command();

        self.file_writer
            .write(format!("Execution simulation completed successfully. Output written to: {}", TEMPORARY_OUTPUT_FILE).as_str())
            .expect("Failed to write log entry");

        self.logger.info(format!("Logs saved to {}", EXECUTION_LOG_FILE));
    }

    fn execute_command(&self)
    {
        let command_to_be_executed = format!("echo 'Simulated command shell execution by attacker' > {}", TEMPORARY_OUTPUT_FILE);

        match Command::new("bash")
            .arg("-c")
            .arg(&command_to_be_executed)
            .output()
        {
            Ok(output) => {
                self.logger.info("Command shell simulation executed successfully.");
                self.logger.verbose(format!("Command output: {}", String::from_utf8_lossy(&output.stdout)));
            }
            Err(e) => {
                self.logger.error(format!("Failed to simulate command shell: {}", e));
            }
        }
    }
}
