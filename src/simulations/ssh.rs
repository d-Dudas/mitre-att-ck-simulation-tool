use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

const SSH_LOG_FILE: &str = "ssh.log";
const LOCALHOST: &str = "127.0.0.1:22";
const USERNAME: &str = "testuser";
const PASSWORD: &str = "password";

pub struct Ssh
{
    file_writer: crate::utils::io::file_writer::FileWriter,
    logger: crate::utils::logger::Logger,
}

impl Ssh {
    pub fn new(results_directory_global_path: &str) -> Self {
        let result_path = format!("{}/{}", results_directory_global_path, SSH_LOG_FILE);
        Ssh {
            file_writer: crate::utils::io::file_writer::FileWriter::new(result_path),
            logger: crate::utils::logger::Logger::new("SSH"),
        }
    }

    pub fn run(&self) {
        self.logger.info("Simulating SSH lateral movement...");

        let maybe_session = self.create_session();
        if maybe_session.is_none() {
            self.logger.error("Failed to create SSH session.");
            return;
        }
        let session = maybe_session.unwrap();

        
        let authenticated = self.authenticate(&session);
        if !authenticated {
            self.logger.error("SSH authentication failed.");
            return;
        }

        let maybe_channel = self.open_channel(&session);
        if maybe_channel.is_none() {
            self.logger.error("Failed to open SSH channel.");
            return;
        }
        let mut channel = maybe_channel.unwrap();

        match self.execute_command(&mut channel, "whoami") {
            Ok(output) => {
                self.logger.info(format!("Command executed successfully. Output: {}", output));
                self.file_writer.write(format!("SSH Command Output: {}", output).as_str())
                    .expect("Failed to write log entry");
            }
            Err(e) => {
                self.logger.error(format!("Command execution failed: {}", e));
            }
        }

        self.close_channel(&mut channel);


        self.logger.info(format!("Logs saved to {}", SSH_LOG_FILE));
    }

    fn create_session(&self) -> Option<Session>
    {
        let tcp = TcpStream::connect(LOCALHOST).expect("Failed to connect to SSH server. Perhaps is not running? 'sudo systemclt start sshd'");
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        
        match session.handshake() {
            Ok(_) => Some(session),
            Err(e) => {
                self.logger.error(format!("SSH handshake failed: {}", e));
                None
            }
        }
    }

    fn authenticate(&self, session: &Session) -> bool {
        if session.userauth_password(USERNAME, PASSWORD).is_err() {
            self.logger.error("Authentication failed");
            return false;
        }

        if !session.authenticated() {
            self.logger.error("SSH session is not authenticated.");
            return false;
        }

        true
    }

    fn open_channel(&self, session: &Session) -> Option<ssh2::Channel> {
        match session.channel_session() {
            Ok(channel) => Some(channel),
            Err(e) => {
                self.logger.error(format!("Failed to open SSH channel: {}", e));
                None
            }
        }
    }

    fn execute_command(&self, channel: &mut ssh2::Channel, command: &str) -> Result<String, String> {
        if channel.exec(command).is_err() {
            return Err(format!("Failed to execute command: {}", command));
        }

        let mut output = String::new();
        if channel.read_to_string(&mut output).is_err() {
            return Err("Failed to read command output".to_string());
        }

        Ok(output)
    }

    fn close_channel(&self, channel: &mut ssh2::Channel) {
        channel.send_eof().unwrap();
        channel.wait_close().unwrap();
        self.logger.info(format!("SSH channel closed with exit code: {}", channel.exit_status().unwrap()));
    }

}
