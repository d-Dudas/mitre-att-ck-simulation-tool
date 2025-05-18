use std::fs::OpenOptions;
use std::io::Write;

pub struct FileWriter {
    file_path: String,
}

impl FileWriter {
    pub fn new(file_path: String) -> Self {
        FileWriter { file_path }
    }

    pub fn write(&self, content: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.file_path)
            .expect("Unable to open log file");

        if let Err(e) = writeln!(file, "{}", content) {
            eprintln!("Couldn't write to file: {}", e);

            return Err(e);
        }

        Ok(())
    }
}
