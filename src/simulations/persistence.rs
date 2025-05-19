use std::process::Command;

use crate::utils::io::file_writer::FileWriter;
use crate::utils::logger::Logger;

const PERSISTENCE_LOG_FILE: &str = "persistence.log";
const FAKE_PAYLOAD_FILE_PATH: &str = "/tmp/fakepayload.sh";
const CRON_ENTRY: &str = "@reboot /tmp/fakepayload.sh\n";
const CRON_DIR: &str = "/var/spool/cron";

pub struct Persistence {
    logger: Logger,
    file_writer: FileWriter,
}

impl Persistence
{
    pub fn new(results_directory_global_path: &str) -> Self
    {
        let result_path = format!("{}/{}", results_directory_global_path, PERSISTENCE_LOG_FILE);
        Persistence {
            file_writer: FileWriter::new(result_path),
            logger: Logger::new("Persistence"),
        }
    }

    pub fn run(&self)
    {
        self.logger.info("Hello from persistence run.");

        if let Err(e) = self.create_executable_script()
        {
            self.logger.error(format!("Failed to create executable script: {}", e));
            return;
        }

        let result = Command::new("bash")
            .arg("-c")
            .arg(format!("(crontab -l 2>/dev/null; echo '{}') | crontab -", CRON_ENTRY.trim()))
            .output();

        match result {
            Ok(_) => self.logger.info("Persistence via cron job simulated."),
            Err(e) => self.logger.error(format!("Failed to modify crontab: {}", e)),
        }

        self.write_log_report();
    }

    fn create_executable_script(&self) -> Result<(), String>
    {
        let write_result = std::fs::write(FAKE_PAYLOAD_FILE_PATH, "#!/bin/bash\necho 'Malware started' >> /tmp/malware.log\n");
        if write_result.is_err()
        {
            return Err(format!("Failed to write to {}", FAKE_PAYLOAD_FILE_PATH));
        }

        let chmod_result = Command::new("chmod").arg("+x").arg(FAKE_PAYLOAD_FILE_PATH).output();
        if chmod_result.is_err()
        {
            return Err(format!("Failed to make {} executable.", FAKE_PAYLOAD_FILE_PATH));
        }

        Ok(())
    }

    fn write_log_report(&self)
    {
        let modified_cron_file = self.get_modified_cron_file_path();
        if modified_cron_file.is_empty()
        {
            self.logger.error("Failed to find modified cron file.");
            return;
        }

        let mut log: String = "".to_string();
        log.push_str(format!("Script installed: {}", FAKE_PAYLOAD_FILE_PATH).as_str());
        log.push_str("\nCrontab entry added:\n");
        log.push_str(CRON_ENTRY);

        log.push_str("\nModified crontab file:\n");
        log.push_str(format!("{}\n", modified_cron_file).as_str());
        
        let result = self.file_writer.write(&log);
        if result.is_err()
        {
            self.logger.error("Failed to write log report.");
            return;
        }

        self.logger.info("Log report succesfuly written.");
    }

    fn get_modified_cron_file_path(&self) -> String
    {
        let cron_files = self.get_files_from_cron_dir();

        for file in cron_files {
            let path = format!("{}/{}", CRON_DIR, file);
            if let Ok(content) = std::fs::read_to_string(&path) {
                if content.contains(CRON_ENTRY.trim()) {
                    return format!("{}/{}", CRON_DIR, file);
                }
            }
        }

        return "".to_string();
    }

    fn get_files_from_cron_dir(&self) -> Vec<String>
    {
        match std::fs::read_dir(CRON_DIR) {
            Ok(entries) => entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| entry.file_name().into_string().ok())
                .collect(),
            Err(e) => {
                self.logger.error(format!("Failed to read {}: {}", CRON_DIR, e));
                Vec::new()
            }
        }
    }
}
