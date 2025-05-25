use std::io::Write;

const DLL_INJECTION_LOG_FILE: &str = "dll_injection.log";
const MALICIOUS_LIB_PATH: &str = "/tmp/libmalicious.so";
const MALICIOUS_LIB_CODE: &str = "/tmp/libmalicious.c";
const MALICIOUS_LIB_OUTPUT_FILE: &str = "/tmp/preload_sim.log";

pub struct DllInjection {
    file_writer: crate::utils::io::file_writer::FileWriter,
    logger: crate::utils::logger::Logger,
    output_log: String,
}

impl DllInjection {
    pub fn new(results_directory_global_path: &str) -> Self {
        let result_path = format!(
            "{}/{}",
            results_directory_global_path, DLL_INJECTION_LOG_FILE
        );
        DllInjection {
            file_writer: crate::utils::io::file_writer::FileWriter::new(result_path),
            logger: crate::utils::logger::Logger::new("DLL Injection"),
            output_log: String::new(),
        }
    }

    pub fn run(&mut self) {
        self.output_log = "Simulating DLL Injection (T1055.001)".to_string();
        self.logger.info("Simulating DLL Injection...");

        if !self.create_malicious_lib() {
            self.logger.error("Failed to create malicious library.");
            return;
        }

        if !self.compile_malicious_lib() {
            self.logger.error("Failed to compile malicious library.");
            return;
        }

        if !self.run_ls_with_ld_preload() {
            self.logger.error("Failed to run command with LD_PRELOAD.");
            return;
        }

        self.file_writer
            .write(&self.output_log)
            .expect("Failed to write log entry");

        self.logger.info(format!(
            "Simulation complete. Log entry saved to {}",
            DLL_INJECTION_LOG_FILE
        ));
    }

    fn create_malicious_lib(&mut self) -> bool {
        let malicious_lib_content = format!(
            "#include <stdio.h>\n\
            #include <stdlib.h>\n\
            \n\
            __attribute__((constructor))\n\
            void preload() {{\n\
                system(\"echo 'LD_PRELOAD MALWARE SIMULATION' >> {}\");\n\
            }}",
            MALICIOUS_LIB_OUTPUT_FILE
        );

        let maybe_file = std::fs::File::create(MALICIOUS_LIB_CODE);
        if maybe_file.is_err() {
            self.logger.error(format!(
                "Failed to create malicious library: {}",
                maybe_file.unwrap_err()
            ));
            return false;
        }

        let mut file = maybe_file.unwrap();
        if let Err(e) = file.write_all(malicious_lib_content.as_bytes()) {
            self.logger
                .error(format!("Failed to write to malicious library: {}", e));
            return false;
        }

        self.logger.info(format!(
            "Malicious library created at: {}",
            MALICIOUS_LIB_CODE
        ));

        self.output_log
            .push_str("\nMalicious library created successfully at: ");
        self.output_log.push_str(MALICIOUS_LIB_CODE);

        return true;
    }

    fn compile_malicious_lib(&mut self) -> bool {
        let output = std::process::Command::new("gcc")
            .arg("-shared")
            .arg("-fPIC")
            .arg("-o")
            .arg(MALICIOUS_LIB_PATH)
            .arg(MALICIOUS_LIB_CODE)
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    self.logger.error(format!(
                        "Failed to compile malicious library: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                    return false;
                }
            }
            Err(e) => {
                self.logger.error(format!("Error compiling library: {}", e));
                return false;
            }
        }

        self.logger.info(format!(
            "Malicious library compiled successfully: {}",
            MALICIOUS_LIB_PATH
        ));

        self.output_log
            .push_str("\nMalicious library compiled successfully at: ");
        self.output_log.push_str(MALICIOUS_LIB_PATH);

        return true;
    }

    fn run_ls_with_ld_preload(&mut self) -> bool {
        let target = "/usr/bin/ls";

        let output = std::process::Command::new(target)
            .env("LD_PRELOAD", MALICIOUS_LIB_PATH)
            .output()
            .expect("Failed to execute target with LD_PRELOAD");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);

            self.logger
                .error(format!("Command execution failed: {}", stderr));
            self.output_log.push_str("\nCommand execution failed: ");
            self.output_log.push_str(&stderr);

            return false;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let malicious_lib_output = self.get_malicious_lib_output_file_content();

        self.logger
            .info(format!("Command executed successfully: {}", target));
        
        self.output_log
            .push_str("\nCommand executed successfully: ");
        self.output_log.push_str(&stdout);
        self.output_log.push_str("\nMalicious library output: ");
        self.output_log.push_str(&malicious_lib_output);

        return true;
    }

    fn get_malicious_lib_output_file_content(&self) -> String {
        let maybe_content = std::fs::read_to_string(MALICIOUS_LIB_OUTPUT_FILE);
        if maybe_content.is_err() {
            self.logger.error(format!(
                "Failed to read malicious library output file: {}",
                maybe_content.unwrap_err()
            ));
            return String::new();
        }

        return maybe_content.unwrap();
    }
}
