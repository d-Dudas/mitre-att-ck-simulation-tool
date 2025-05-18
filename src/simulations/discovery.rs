use std::process::Command;
use chrono::Local;

use crate::utils::file_writer::FileWriter;

use crate::{log, log_error, log_verbose};

const DISCOVERY_LOG_FILE: &str = "discovery.log";

pub struct Discovery {
    file_writer: FileWriter,
}

impl Discovery {
    pub fn new(results_directory_global_path: &str) -> Self {
        let result_path = format!("{}/{}", results_directory_global_path, DISCOVERY_LOG_FILE);
        Discovery {
            file_writer: FileWriter::new(result_path),
        }
    }

    pub fn run(&self) {
        log!("[Discovery / T1087] Simulating Account Discovery...");

        let output = Command::new("getent").arg("passwd").output();

        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let mut log_entry = format!("[{}] Discovery (T1087): ", timestamp);

        match output {
            Ok(result) => {
                if result.status.success() {
                    let stdout = String::from_utf8_lossy(&result.stdout);
                    log_verbose!("{}", stdout);
                    log_entry.push_str("Command successful\n");
                    log_entry.push_str(&stdout);
                } else {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    log_error!("Command failed: {}", stderr);
                    log_entry.push_str("Command failed\n");
                    log_entry.push_str(&stderr);
                }
            }
            Err(e) => {
                log_error!("Error executing command: {}", e);
                log_entry.push_str(&format!("Execution error: {}\n", e));
            }
        }

        self.file_writer
            .write(&log_entry)
            .expect("Failed to write log entry");

        log!("[Discovery / T1087] Simulation complete.");
        log!("[Discovery / T1087] Log entry saved to {}", DISCOVERY_LOG_FILE);
    }
}
