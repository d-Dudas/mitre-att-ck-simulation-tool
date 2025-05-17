use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

use crate::log;
use crate::log_error;
use crate::log_verbose;

pub struct Discovery {}

impl Discovery {
    pub fn new() -> Self {
        Discovery {}
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

        // Save to log file
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("simulation.log")
            .expect("Unable to open log file");

        if let Err(e) = writeln!(file, "{}", log_entry) {
            log_error!("Couldn't write to log file: {}", e);
        }

        log!("[Discovery / T1087] Simulation complete.");
        log!("[Discovery / T1087] Log entry saved to simulation.log");
    }
}
